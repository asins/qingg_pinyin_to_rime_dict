use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

lazy_static::lazy_static! {
    // 声母列表（包含复合声母）
    static ref INITIALS: HashSet<&'static str> = [
        "b","p","m","f","d","t","n","l","g","k","h",
        "j","q","x","zh","ch","sh","r","z","c","s","y","w"
    ].iter().cloned().collect();

    // 完整韵母列表（按长度降序排列）
    static ref FINALS: Vec<&'static str> = vec![
        "iang","iong","uang","ueng","ang","eng","ing","ong",
        "uan","uai","iao","ian","iu","ui","un","uo","ua","ve",
        "ai","ei","ao","ou","an","en","ia","ie","in","er",
        "a","o","e","i","u","v"
    ];

    // 可独立存在的韵母（如er）
    static ref STANDALONE_FINALS: HashSet<&'static str> = [
        "a","o","e","ai","ei","ao","ou","an","en","ang",
        "eng","er","yi","wu","yu"
    ].iter().cloned().collect();
}

fn split_pinyin(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut result = Vec::new();
    let mut pos = 0;
    let len = chars.len();

    while pos < len {
        let mut matched = false;

        // 优先处理复合声母（zh/ch/sh）
        if pos + 2 <= len {
            let candidate: String = chars[pos..pos + 2].iter().collect();
            if INITIALS.contains(candidate.as_str()) {
                result.push(candidate);
                pos += 2;
                matched = true;
            }
        }

        if !matched {
            // 处理单声母或独立韵母
            for &final_str in FINALS.iter() {
                let end = pos + final_str.len();
                if end <= len {
                    let candidate: String = chars[pos..end].iter().collect();
                    if (pos == 0 || STANDALONE_FINALS.contains(final_str)) && candidate == final_str
                    {
                        result.push(candidate);
                        pos = end;
                        matched = true;
                        break;
                    }
                }
            }
        }

        if !matched {
            // 处理声母+韵母组合
            if let Some((initial, final_part)) = split_initial_final(&chars[pos..]) {
                result.push(format!("{}{}", initial, final_part));
                pos += initial.len() + final_part.len();
            } else {
                // 保底处理单字符
                result.push(chars[pos].to_string());
                pos += 1;
            }
        }
    }

    println!("{}\t=>\t{}", input, result.join(" "));
    result.join(" ")
}

fn split_initial_final(chars: &[char]) -> Option<(String, String)> {
    // 查找最长可能的声母（1-2字符）
    for initial_len in (1..=2).rev() {
        if chars.len() < initial_len {
            continue;
        }

        let initial: String = chars[..initial_len].iter().collect();
        if INITIALS.contains(initial.as_str()) {
            // 查找最长匹配的韵母
            for final_len in (1..=4).rev() {
                if chars.len() < initial_len + final_len {
                    continue;
                }

                let final_str: String =
                    chars[initial_len..initial_len + final_len].iter().collect();
                if FINALS.contains(&final_str.as_str()) {
                    return Some((initial, final_str));
                }
            }
        }
    }
    None
}

fn process_line(line: &str, writer: &mut BufWriter<File>) -> std::io::Result<()> {
    let trimmed = line.trim();
    if trimmed.is_empty() || !trimmed.starts_with(|c: char| c.is_ascii_lowercase()) {
        return Ok(());
    }

    // 修正点1：正确分割拼音和词组部分
    let (pinyin_part, words_part) = match trimmed.split_once(' ') {
        Some((p, w)) => (p, w),
        None => return Ok(()),
    };

    let words: Vec<&str> = words_part.split_whitespace().collect();
    let word_count = words.len() - 1;
    for (idx, word) in words.iter().enumerate() {
        let py = split_pinyin(pinyin_part);

        let reverse_idx = word_count - idx; // 倒序索引计算
        writeln!(writer, "{}\t{}\t{}", word, py, reverse_idx)?;
    }

    Ok(())
}

pub fn main() -> std::io::Result<()> {
    let input = BufReader::with_capacity(8 * 1024 * 1024, File::open("./pinyin_mergin.dict.yaml")?);
    let output =
        BufWriter::with_capacity(8 * 1024 * 1024, File::create("./pinyin_simp.dict.yaml")?);

    let mut writer = BufWriter::new(output.into_inner()?);

    let str = r###"# Rime dictionary
# encoding: utf-8
#
# A minimal Pinyin dictionary for simplified Chinese script
#
# Derived from android open source project:
# http://android.git.kernel.org/?p=platform/packages/inputmethods/PinyinIME.git
#

---
name: pinyin_simp
version: "0.1"
sort: by_weight
..."###;
    writeln!(writer, "{}", str)?;

    for line in input.lines() {
        process_line(&line?, &mut writer)?;
    }

    writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_pinyin() {
        assert_eq!("a gen ting dui", split_pinyin("agentingdui")); // 阿根廷队
        assert_eq!("xian", split_pinyin("xian"));
        assert_eq!("nv hai", split_pinyin("nvhai"));
    }
}

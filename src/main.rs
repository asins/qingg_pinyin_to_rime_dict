use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::sync::LazyLock;

static VALID_PINYINS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    let mut set = HashSet::with_capacity(420);
    // 基础单韵母（6个）
    set.extend(["a", "o", "e", "i", "u", "v"]);

    // 复韵母（13个）
    set.extend([
        "ai", "ei", "ui", "ao", "ou", "iu", "ie", "ve", "er", "an", "en", "in", "un",
    ]);

    // 鼻韵母（16个）
    set.extend([
        "ang", "eng", "ing", "ong", "ian", "uan", "van", "iang", "uang", "iong", "uen", "van",
        "vn", "ang", "eng", "ing",
    ]);

    // 声母组合音节（按声母分组）
    // b/p/m/f 系列
    set.extend([
        "ba", "bo", "bai", "bei", "bao", "ban", "ben", "beng", "bi", "bian", "biao", "bie", "bin",
        "bing", "bu", "pa", "po", "pai", "pei", "pao", "pou", "pan", "pen", "peng", "pi", "pian",
        "piao", "pie", "pin", "ping", "pu", "ma", "mo", "me", "mai", "mao", "mou", "man", "men",
        "mang", "meng", "mi", "mian", "miao", "mie", "min", "ming", "mu", "fa", "fo", "fei", "fou",
        "fan", "fen", "fang", "feng",
    ]);

    // d/t/n/l 系列
    set.extend([
        "da", "de", "dai", "dei", "dao", "dou", "dan", "dang", "deng", "di", "dia", "dian", "diao",
        "die", "ding", "diu", "dong", "du", "duan", "dui", "dun", "duo", "ta", "te", "tai", "tao",
        "tou", "tan", "tang", "teng", "ti", "tian", "tiao", "tie", "ting", "tong", "tu", "tuan",
        "tui", "tun", "tuo", "na", "ne", "nai", "nao", "nan", "nang", "nen", "neng", "ni", "nia",
        "nian", "niang", "niao", "nie", "nin", "ning", "niu", "nong", "nu", "nv", "nuan", "nuo",
        "la", "le", "lai", "lao", "lan", "lang", "leng", "li", "lia", "lian", "liang", "liao",
        "lie", "lin", "ling", "liu", "long", "lou", "lu", "lv", "luan", "lun", "luo",
    ]);

    // g/k/h 系列
    set.extend([
        "ga", "ge", "gai", "gei", "gao", "gan", "gen", "geng", "gong", "gou", "gu", "gua", "guai",
        "guan", "guang", "gui", "gun", "guo", "ka", "ke", "kai", "kao", "kan", "ken", "keng",
        "kong", "kou", "ku", "kua", "kuai", "kuan", "kuang", "kui", "kun", "kuo", "ha", "he",
        "hai", "hei", "hao", "han", "hen", "heng", "hong", "hou", "hu", "hua", "huai", "huan",
        "huang", "hui", "hun", "huo",
    ]);

    // j/q/x 系列
    set.extend([
        "ji", "jia", "jian", "jiang", "jiao", "jie", "jin", "jing", "jiong", "jiu", "ju", "juan",
        "jue", "jun", "qi", "qia", "qian", "qiang", "qiao", "qie", "qin", "qing", "qiong", "qiu",
        "qu", "quan", "que", "qun", "xi", "xia", "xian", "xiang", "xiao", "xie", "xin", "xing",
        "xiong", "xiu", "xu", "xuan", "xue", "xun",
    ]);

    // zh/ch/sh/r 系列
    set.extend([
        "zha", "zhe", "zhi", "zhai", "zhao", "zhou", "zhan", "zhen", "zhang", "zheng", "zhong",
        "zhu", "zhua", "zhuai", "zhuan", "zhuang", "zhui", "zhun", "zhuo", "cha", "che", "chi",
        "chai", "chao", "chou", "chan", "chen", "chang", "cheng", "chong", "chu", "chua", "chuai",
        "chuan", "chuang", "chui", "chun", "chuo", "sha", "she", "shi", "shai", "shao", "shou",
        "shan", "shen", "shang", "sheng", "shu", "shua", "shuai", "shuan", "shuang", "shui",
        "shun", "shuo", "re", "ri", "rao", "rou", "ran", "ren", "rang", "reng", "rong", "ru",
        "rua", "ruan", "rui", "run", "ruo",
    ]);

    // z/c/s 系列
    set.extend([
        "za", "ze", "zi", "zai", "zao", "zou", "zan", "zen", "zang", "zeng", "zong", "zu", "zuan",
        "zui", "zun", "zuo", "ca", "ce", "ci", "cai", "cao", "cou", "can", "cen", "cang", "ceng",
        "cong", "cu", "cuan", "cui", "cun", "cuo", "sa", "se", "si", "sai", "sao", "sou", "san",
        "sen", "sang", "seng", "song", "su", "suan", "sui", "sun", "suo",
    ]);

    // 整体认读音节（16个）
    set.extend([
        "zhi", "chi", "shi", "ri", "zi", "ci", "si", "yi", "wu", "yu", "ye", "yue", "yuan", "yin",
        "yun", "ying",
    ]);

    // 特殊组合
    set.extend(["yo", "lo", "fiao", "n", "ng", "hm", "hng", "m"]);

    set
});

pub fn split_pinyin(s: &str, n: usize) -> String {
    // 获取所有可能的分割方案
    let mut splits = backtrack(s, n, &VALID_PINYINS);

    // 返回第一个有效分割（或空字符串）
    splits.pop().map_or(String::new(), |v| v.join(" "))
}

// 回溯算法实现
fn backtrack(s: &str, k: usize, dict: &HashSet<&str>) -> Vec<Vec<String>> {
    if k == 0 || s.is_empty() {
        return vec![];
    }
    if k == 1 {
        return if dict.contains(s) {
            vec![vec![s.to_string()]]
        } else {
            vec![]
        };
    }

    let max_len = s.len().saturating_sub(k - 1);
    let mut result = vec![];

    // 从长到短尝试前缀
    for i in (1..=max_len).rev() {
        let prefix = &s[..i];
        if dict.contains(prefix) {
            let rest = &s[i..];
            let mut sub_splits = backtrack(rest, k - 1, dict);
            for mut sub_split in sub_splits.drain(..) {
                sub_split.insert(0, prefix.to_string());
                result.push(sub_split);
            }
        }
    }
    result
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
        let py = split_pinyin(pinyin_part, word.chars().count());

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
    fn test_chinese_word_length() {
        assert_eq!("你好".chars().count(), 2);
    }

    #[test]
    fn test_common_pinyins() {
        assert!(VALID_PINYINS.contains("a"));
        assert!(VALID_PINYINS.contains("nv")); // ü 写作 v
        assert!(VALID_PINYINS.contains("lv")); // ǖ 写作 v
        assert!(VALID_PINYINS.contains("xian")); // 三拼音节
        assert!(VALID_PINYINS.contains("zhuang")); // 复合鼻韵母
    }

    #[test]
    fn test_split_pinyin() {
        assert_eq!(split_pinyin("agentingdui", 4), "a gen ting dui"); // 阿根廷队
        assert_eq!(split_pinyin("nvhai", 2), "nv hai");

        assert_eq!(split_pinyin("xian", 1), "xian"); // 单字优先
        assert_eq!(split_pinyin("xian", 2), "xi an"); // 强制双字切分
        assert_eq!(split_pinyin("zhongguo", 2), "zhong guo");
    }
}

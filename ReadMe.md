# 将清歌输入法的拼音词库转换为Rime输入法的拼音词库

清歌输入法的词库格式为 `无空格词语拼音\s词语1\s词语2\s词语3...`

如：
```yaml
abasi 阿巴斯
bian 扁
yuanchengdenglu 远程登陆 远程登录
```

而Rime输入法的词库格式为 `词语\t用空格分隔的汉字拼音\t词频`
如：
```yaml
---
name: pinyin_simp
version: "0.1"
sort: by_weight
...
阿巴斯	a ba si	0
扁	bian	0
远程登陆 yuan cheng deng lu	1
远程登录 yuan cheng deng lu	0
```

所以上面转换的重点是清歌输入法的词库中无空格分隔的拼音字符串如何标准的为其加入空格来分隔开。

## 构建生成

执行Cargo run命令，会在当前目录下生成名为`pinyin_simp.dict.yaml`的文件。

## TODO

- 后面有时间了来支持下`https://github.com/mozillazg/phrase-pinyin-data.git`仓库中的`large_pinyin.txt`字典的转换
- 汉字转拼音工具，支持多音词：https://github.com/duguying/pinyin-translator

# README
[![zh-Hans](https://img.shields.io/badge/-%E7%AE%80%E4%BD%93%E4%B8%AD%E6%96%87-black.svg?style=for-the-badge&logo=googletranslate&logoColor=yellow)](https://github.com/Hambaka/golden_sun_password_converter/blob/main/README.md)
[![en-US](https://img.shields.io/badge/-English-black.svg?style=for-the-badge&logo=googletranslate&logoColor=yellow)](https://github.com/Hambaka/golden_sun_password_converter/blob/main/README.en-US.md)
---
# golden_sun_password_converter （已过时）

![Rust](https://img.shields.io/badge/language-Rust-DEA584.svg?style=flat-square&logo=rust)
[![GitHub license](https://img.shields.io/github/license/Hambaka/golden_sun_password_converter?style=flat-square)](https://raw.githubusercontent.com/Hambaka/golden_sun_password_converter/master/LICENSE)
![Platform](https://img.shields.io/badge/platform%20(x86--64)-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey?style=flat-square)
[![Version](https://img.shields.io/github/v/release/Hambaka/golden_sun_password_converter?label=version&style=flat-square)](https://github.com/Hambaka/golden_sun_password_converter/releases/latest)  
 
　　黄金太阳密码文本转换工具（英文转日文，日文转英文）  

## 重要提示

日期：2023 年 4 月 24 日  

　　本仓库提供的 `golden_sun_password_converter` 已是**过时**的工具，如果要转换密码文本到另外一个版本，请见 [golden_sun_password_exporter](https://github.com/Hambaka/golden_sun_password_exporter) 仓库提供的程序。  
　　可以使用其子命令 `txt` 来转换密码文本的版本：  
```shell
golden_sun_password_exporter txt --text <要转换的密码文本文件>
# 或者也可以使用下面的格式 ↓
golden_sun_password_exporter txt -t <要转换的密码文本文件>
```
　　默认情况下，程序会创建一个新的文件夹，文件夹的名字是输入的文本文件名+ `_output` ，并将转换好的密码文本文件存储在该文件夹中。  

## 这是啥玩意儿？

　　一个简单到不能再简单的小工具，主要用于 GBA 上的《黄金太阳》系列的两个游戏，也就是《黄金太阳 开启的封印》和《黄金太阳 失落的时代》。  
　　如果通关了《黄金太阳 开启的封印》，可以获得通关存档的密码，并能在其续作《黄金太阳 失落的时代》中输入获得的密码来继承游戏数据。  
　　日文版的密码和英文版的密码看上去***不同***，但本质上是完全***相同***的。有时候玩家可能会有转换密码版本的需求。  


## 那该咋用呢？

　　将程序以及一个文件名为 `input.txt` 的密码文本文件放在同一个文件夹下，然后运行程序即可，转换好的文件 `output.txt` 也会输出在同一文件夹下。  
- 若输入的密码文本文件是英文版密码，则输出的密码文本文件是日文版密码。  
- 若输入的密码文本文件是日文版密码，则输出的密码文本文件是英文版密码。  

**注意事项：`input.txt` 的编码必须是 `UTF-8`！**  

## 来个示例呗？

用于演示的 `input.txt` 文件  

```
w$wZZ P!Vm#
Qndtz qKjF=
D2YnP J2RkJ
baHyQ ZPTqe
HSYq! XD6%j

wBMt% SmYzM
Y?Mcp tXtsX
W2Y6c !yxJa
ZB&PP JCJVz
TgFcK aN9nv

+yPHE TGcpz
JSb5d RfGz7
V$6ba Yy6LD
H&zb8 9j3Dt
VA8p? pD$#C

22kAw zVkTJ
$8rYb bv4EZ
b!9Kf $dPjS
BiRqp 5yUKE
t5zQx 9&U?a

eA3c+ EV#6r
KaHtP eMxT!
jS#X% W+YfZ
bD9y7 Hd#bS
Ni=gT XPxW#

tJr3x Md7uJ
```

`output.txt` （输出结果是日文版密码）  

```
じびじねね せだとわど
そんやげぞ がこれかぼ
えのぬんせ けのたろけ
めむくぜそ ねせつがゆ
くちぬがだ にえへぶれ

じいしげぶ ちわぬぞし
ぬでしもを げにげぐに
なのぬへも だぜずけむ
ねいばせせ けうけとぞ
つらかもこ むすみんざ

べぜせくお つきもをぞ
けちめふや たよきぞほ
とびへめむ ぬぜへさえ
くばぞめま みれはえげ
とあまをで をえびどう

ののろあじ ぞとろつけ
びまぎぬめ めざひおね
めだみこよ びやせれち
いるたがを ふぜてこお
げふぞそず みばてでむ

ゆあはもべ おとどへぎ
こむくげせ ゆしずつだ
れちどにぶ なべぬよね
めえみぜほ くやどめち
するぼらつ にせずなど

げけぎはず しやほごけ
```

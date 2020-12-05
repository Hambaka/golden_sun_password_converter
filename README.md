# golden_sun_password_converter

![Rust](https://img.shields.io/badge/language-Rust-DEA584.svg?style=flat-square)
[![GitHub license](https://img.shields.io/github/license/Hambaka/golden_sun_password_converter?style=flat-square)](https://raw.githubusercontent.com/Hambaka/golden_sun_password_converter/master/LICENSE)
[![Version](https://img.shields.io/github/v/release/Hambaka/golden_sun_password_converter?label=version&style=flat-square)](https://github.com/Hambaka/golden_sun_password_converter/releases/latest)

Golden Sun password converter (English -> Japanese, Japanese -> English)

## What's this?

A really simple tool for two Game Boy Advance games, Golden Sun (黄金の太陽 開かれし封印) and Golden Sun: The Lost Age (黄金の太陽 失われし時代).
If you clear Golden Sun, you can get password of your clear data, and you can transfer your game data to Golden Sun: The Lost Age by inputting password.  
Japanese version password and English version password ***looks different***, but they are ***actually identical***. Sometimes maybe you want to convert your password to another version because of some reasons.


## Usage

Put the binary file and a text file called `input.txt` that contains Golden Sun password in the same folder, then run binary. It will output `output.txt` in the same folder.
- If input file is English version, then output file is Japanese version.
- If input file is Japanese version, then output file is English version.

**Note: The encoding of `input.txt` must be `UTF-8`.**

## Example

Example `input.txt` (English version password as source)

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

`output.txt` (result is Japanese version password)

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

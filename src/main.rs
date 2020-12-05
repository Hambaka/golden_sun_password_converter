use std::io::Read;
use std::io::Write;
use std::fs::File;

enum SaveType {
  English,
  Japanese,
}

/// Golden Sun password converter (English -> Japanese, Japanese -> English)
/// Hambaka <readeryoshi@gmail.com>
/// Port Java version to Rust version with help from my friend "Flat Bartender", a huge thanks to him!
fn main() {
  let mut input_file = File::open("input.txt").expect("An error happened while opening input.txt");
  let mut password = String::new();
  input_file.read_to_string(&mut password).unwrap();

  // If the file is empty, exit program.
  if password.is_empty() {
    println!("\"input.txt\" is empty!");
    return;
  }

  // Get converted password.
  // If input file is English version, then output file is Japanese version.
  // If input file is Japanese version, then output file is English version.
  let converted_password: String = match get_save_type(password.as_ref()) {
    SaveType::English => password.chars().map(convert_en_to_jp).collect(),
    SaveType::Japanese => password.chars().map(convert_jp_to_en).collect(),
  };

  // After converting, the result is "output.txt" in the same folder.
  // If "output.txt" was already existed, clear all content in the file and write converted password.
  // If not, create file and write converted password.
  let mut output_file = std::fs::File::create("output.txt").expect("Create \"output.txt\" failed!");
  output_file.write_all(converted_password.as_bytes()).expect("Write to \"output.txt\" failed!");
  println!("Conversion is completed! Please check out \"output.txt\".");
}

/// Convert English version password (letters, numbers, signs) to Japanese version password (hiragana).
fn convert_en_to_jp(input: char) -> char {
  match input {
    'A' => 'あ',
    'B' => 'い',
    'C' => 'う',
    'D' => 'え',
    'E' => 'お',

    'F' => 'か',
    'G' => 'き',
    'H' => 'く',
    'J' => 'け',
    'K' => 'こ',

    'L' => 'さ',
    'M' => 'し',
    'N' => 'す',
    'P' => 'せ',
    'Q' => 'そ',

    'R' => 'た',

    'S' => 'ち',
    'T' => 'つ',
    'U' => 'て',
    'V' => 'と',
    'W' => 'な',

    'X' => 'に',
    'Y' => 'ぬ',
    'Z' => 'ね',
    '2' => 'の',
    '3' => 'は',

    '4' => 'ひ',
    '5' => 'ふ',
    '6' => 'へ',
    '7' => 'ほ',
    '8' => 'ま',

    '9' => 'み',

    'a' => 'む',
    'b' => 'め',
    'c' => 'も',
    'd' => 'や',
    'e' => 'ゆ',

    'f' => 'よ',
    'g' => 'ら',
    'h' => 'り',
    'i' => 'る',
    'j' => 'れ',

    'k' => 'ろ',
    'm' => 'わ',
    'n' => 'ん',
    'p' => 'を',
    'q' => 'が',

    'r' => 'ぎ',

    's' => 'ぐ',
    't' => 'げ',
    'u' => 'ご',
    'v' => 'ざ',
    'w' => 'じ',

    'x' => 'ず',
    'y' => 'ぜ',
    'z' => 'ぞ',
    '!' => 'だ',
    '?' => 'で',

    '#' => 'ど',
    '&' => 'ば',
    '$' => 'び',
    '%' => 'ぶ',
    '+' => 'べ',

    '=' => 'ぼ',
    _ => input,
  }
}

/// Convert Japanese version password (hiragana) to English version password (letters, numbers, signs).
fn convert_jp_to_en(input: char) -> char {
  match input {
    'あ' => 'A',
    'い' => 'B',
    'う' => 'C',
    'え' => 'D',
    'お' => 'E',

    'か' => 'F',
    'き' => 'G',
    'く' => 'H',
    'け' => 'J',
    'こ' => 'K',

    'さ' => 'L',
    'し' => 'M',
    'す' => 'N',
    'せ' => 'P',
    'そ' => 'Q',

    'た' => 'R',
    'ち' => 'S',
    'つ' => 'T',
    'て' => 'U',
    'と' => 'V',

    'な' => 'W',
    'に' => 'X',
    'ぬ' => 'Y',
    'ね' => 'Z',
    'の' => '2',

    'は' => '3',
    'ひ' => '4',
    'ふ' => '5',
    'へ' => '6',
    'ほ' => '7',

    'ま' => '8',
    'み' => '9',
    'む' => 'a',
    'め' => 'b',
    'も' => 'c',

    'や' => 'd',
    'ゆ' => 'e',
    'よ' => 'f',

    'ら' => 'g',
    'り' => 'h',
    'る' => 'i',
    'れ' => 'j',
    'ろ' => 'k',

    'わ' => 'm',
    'を' => 'p',
    'ん' => 'n',

    'が' => 'q',
    'ぎ' => 'r',
    'ぐ' => 's',
    'げ' => 't',
    'ご' => 'u',

    'ざ' => 'v',
    'じ' => 'w',
    'ず' => 'x',
    'ぜ' => 'y',
    'ぞ' => 'z',

    'だ' => '!',
    'で' => '?',
    'ど' => '#',

    'ば' => '&',
    'び' => '$',
    'ぶ' => '%',
    'べ' => '+',
    'ぼ' => '=',
    _ => input,
  }
}

/// Very stupid and lazy way to detect content (English version password or Japanese version password).
/// By comparing first char to 'z' in ASCII. Because 'z' is the "largest" char in English version password.
/// If the first char is larger than 'z', then it is Japanese version password, since Japanese version password are all hiragana.
fn get_save_type(password: &str) -> SaveType {
  let first_char = password.chars().nth(0).unwrap();
  if first_char <= 'z' {
    SaveType::English
  } else {
    SaveType::Japanese
  }
}

use base64;
use itertools::zip;
use regex::Regex;
use std::collections::HashMap;
use std::process;

pub struct Rule {
    dict: HashMap<char, char>,
}

impl Rule {

    pub fn new(dic_vec: Vec<char>) -> Rule {
        // check if rule is valid
        check(dic_vec.clone());

        let mut dict = HashMap::new();
        let base = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";
        for (dc, bs) in zip(dic_vec.into_iter(), base.chars()) {
            dict.insert(bs, dc);
            // make it easy for encoding.
            dict.insert(dc, bs);
            // make it easy for decoding.

            // (just too lazy to realize looking up between 2 arrays)
        }
        Rule { dict }
    }

    fn cover(&self, raw: String) -> String {
        // render base64 encoded string
        let mut outcome = String::from("犬曰：");

        for ch in raw.chars() {
            // hash map fast lookup.
            outcome.push(
                // Don't tell me char has thing to do with *OWNERSHIP*
                *self.dict.get(&ch)
                    .unwrap_or_else(|| {
                    eprintln!("在从 base64 转成编码数据时发生错误：\n");
                    process::exit(1);
                }),
            );
        }

        outcome
    }

    fn uncover(&self, encoded: String) -> String {
        let mut outcome = String::new();

        let rg = Regex::new("^犬曰：").unwrap();
        if !rg.is_match(outcome.as_str()) {
            eprintln!("🐕️：我说的？\n需要以“犬曰：”开头。")
        }

        for ch in encoded.chars() {
            outcome.push(*self.dict.get(&ch).unwrap_or_else(|| {
                eprintln!(
                    "在从已编码数据转换成 base64 时出错。\n建议检查是否有不正确的字符。\n"
                );
                process::exit(1);
            }));
        }

        outcome
    }

    pub fn encrypt(&self, plain: String) -> String {
        let b64 = base64::encode(plain);

        self.cover(b64)
    }

    pub fn decrypt(&self, encrypted: String) -> String {
        let b64 = self.uncover(encrypted);

        let raw_num = base64::decode(b64).unwrap_or_else(|err| {
            eprintln!("base64 解码出错");
            process::exit(1);
        });

        String::from_utf8(raw_num).unwrap()
    }
}

fn check(dict: Vec<char>) {
    // check if the dictionary is valid
    // 1. length of dictionary should equals 64. (64 characters should be included)
    // 2. the dictionary contains no repeat items. (all characters should be different)
    assert_eq!(dict.len(), 65);
    let mut vc = Vec::from(dict);
    vc.sort();
    vc.dedup();
    assert_eq!(vc.len(), 65);
}
//fn cover(raw: String) -> String{
//     ///Rend base64 encoded string
//     ///simple replacements
//     ///rules: replace fr[i] with to[i]
//     ///fr -> collection of patterns to be replaced
//     /// to -> collection of patterns to be replaced to
//     let fr = Vec![];
//     let to = Vec![];
//     "犬曰：" + replace(raw, fr, to)
// }
//
// fn uncover(rendered: String) -> String{
//     /// Turn rendered text back to base64 encoded string
//     /// simple replacements
//     /// rules: replace to[i] with fr[i]
//     /// fr -> collection of patterns replaced
//     /// to -> collection of raw base64 pattern
//
//     let fr = Vec![];
//     let to = Vec![];
//     // check if begin with specific pattern.
//     let rg = regex::Regex::new(r"^犬曰：").unwrap();
//     if !rg.is_match(rendered.as_str()){
//         eprintln!("🐕️：我没说过这话嗷\n需要以 \"犬曰：\" 开头");
//         process::exit(1);
//     }
//
//     let rendered = rg.replace(rendered.as_str(), "");
//
//     String::from(rendered)
// }
//
// pub fn encrypt(plain: String) -> String{
//     /// translate the plain text to doggy text.
//     let enc_text = encode(plain);
//
//     cover(enc_text)
//     // turn encrypted text to doggy text.
// }
//
// pub fn decrypt(encrypted: String) -> String{
//     /// translate doggy text to plain text
//     let enc_text = uncover(encrypted);
//     let plain_num = decode(enc_text).unwrap_or_else(|err|{
//         eprintln!("Problem occurred while decoding:\n{}", err);
//         process::exit(1);
//     });
//     let plain_text = String::from_utf8(plain_num).unwrap_or_else(|err|{
//         eprintln!("Problem occurred while decoding:\n{}", err);
//         process::exit(1);
//     });
//
//     plain_text
// }
//
// fn replace(raw: String, v_from: Vec<&str>, v_to: Vec<&str>) -> String{
//     /// enforce the replace function of std::str
//     /// making it possible to replace patterns by a single function.
//     if v_from.len() != v_to.len() {
//         eprintln!("Invalid Rule!");
//         process::exit(1);
//     }
//
//     let outcome = raw.clone();
//
//     for (fr, to) in zip(v_from.iter(),v_to.iter()){
//         outcome.replace(fr, to);
//     }
//
//     outcome
// }

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
        if rg.is_match(encoded.as_str()) == false {
            eprintln!("🐕️：我说的？\n需要以“犬曰：”开头。")
        }

        let encoded = String::from(rg.replace(encoded.as_str(),""))
            .replace("\n","");

        for ch in encoded.chars() {
            outcome.push(
                *self.dict.get(&ch)
                .unwrap_or_else(|| {
                eprintln!(
                    "在从已编码数据转换成 base64 时出错。\n建议检查是否有不正确的字符:{}",
                    ch as u32
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


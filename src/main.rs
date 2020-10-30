use doggy::Rule;
use std::env;
use std::process;
use std::fs;

fn main() {
    let ops = env::args()
        .nth(1)
        .expect("请告诉 Doggy 怎么做哦！\n命令格式：doggy_encrypt <操作> <目标文件>");
    let arg = env::args()
        .nth(2)
        .expect("Doggy 需要数据源！\n命令格式：doggy_encrypt <操作> <目标文件>");

    let file = arg;
    // considering realizing reading *Data* directly from console.

    let dictionary =
        String::from(
            // base64 replacement
            String::from("噢奔呲噔呃吠咯哈噫叽控溜哞㖸喔屁气日斯踏呜愈卧吓哟爪") + // Upper case
            "嗷蹦冲瞪耳翻拱哗哩啾跨来弄铃哇跑球人撒舔舞旅挖啸嘤抓" + // lower case
            "窝嚎挠吐撞扯拉捏咩嘻"+ //numbers
            "爬扒罢"  //operators
        );

    let dog = Rule::new(dictionary.chars().collect());
    // encrypt class
    let mut outcome = String::new();
    // outcome restore here ⬆️️
    match ops.as_str() {
        "encrypt" => {
            let raw_data = fs::read_to_string(file)
                .unwrap_or_else(|err|{
                    eprintln!("Doggy 打开文件出错:\n{}",err);
                    process::exit(1);
                });

            outcome = dog.encrypt(raw_data);
        }
        "decrypt" => {
            let enc_data = fs::read_to_string(file)
                .unwrap_or_else(|err|{
                eprintln!("Doggy 打开文件出错:\n{}",err);
                process::exit(1);
                });

            outcome = dog.decrypt(enc_data);
        }
        _ => {
            tutorial();
            process::exit(1);
        }
    }

    print!("{}", outcome);
}

fn tutorial(){
    eprintln!("命令格式：");
    eprintln!();
}
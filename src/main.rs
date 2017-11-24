// use std::thread;
// use std::sync::mpsc::{Sender, Receiver};
// use std::time;

fn main() {
    // TODO: 引数読み込み -> 検索へ
    // TODO: 入力
    // TODO: 検索

    // see: https://doc.rust-lang.org/regex/regex/index.html
    //
    // [マッチしたかどうか（boolean）]
    // use regex::Regex;
    // let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    // assert!(re.is_match("2014-01-01"));
    //
    // [マッチした場所（indexes）]
    // let re = Regex::new(r"[\pN\p{Greek}\p{Cherokee}]+").unwrap();
    // let mat = re.find("abcΔᎠβⅠᏴγδⅡxyz").unwrap();
    // assert_eq!((mat.start(), mat.end()), (3, 23));

    // TODO: 出力
    // TODO: 色つけ
    // see: https://github.com/ogham/rust-ansi-term
    println!("Hello, world!");
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     fn it_works() {
//         assert!(false);
//     }
// }

#[macro_use]
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;
// lazy_static! {
//     static ref HASHMAP: HashMap<&str, &'static str> = {
//         let mut m = HashMap::new();
//         let file_path = "C:\\Users\\magib\\Desktop\\vizApp\\src\\lib_static\\word_tag.json";

//         let json = fs::read_to_string(file_path).expect("Should have been able to read the file");

//         m.insert("word_json", json);
//         m
//     };
// }
use std::fs::read_to_string;

fn read_file_and_return_static_str(filename: &str) -> &'static str {
    // 读取文件内容到一个 String
    let contents = read_to_string(filename).expect("Should have been able to read the file");
    // 将 String 转换成 Box<str>
    let boxed = contents.into_boxed_str();
    // 泄露 Box<str> 的所有权，返回一个 &'static str
    let leaked = Box::leak(boxed);
    // 返回 &'static str
    leaked
}
lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let file_path = "C:\\Users\\magib\\Desktop\\appx\\src\\lib_static\\word_tag.json";

        let txt = read_file_and_return_static_str(&file_path);
        let mut m = HashMap::new();

        m.insert(0, txt);

        m
    };
}

pub fn static_json_txt() -> &'static str {
    // let file_path = "/root/appx/src/lib_static/word_tag.json";

    *HASHMAP.get(&0).unwrap()
}

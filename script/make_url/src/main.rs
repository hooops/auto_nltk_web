use serde_json;
use std::fs;
fn main() {
    let file_path = "C:\\Users\\magib\\Desktop\\vizApp\\src\\lib_static\\word_tag.json";

    // 用read_to_string函数来把文件的内容读取成一个String
    // 如果读取失败，就打印出错误信息
    let json = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let value: serde_json::Value = serde_json::from_str(&json).unwrap();

    // 用get方法来根据键来获取值
    let name = value.get("NN");

    println!("Name: {:?}", name.expect("REASON").get(0).unwrap());
}

use std::fs::read_to_string;

pub fn read_file_and_return_static_str(filename: &str) -> &'static str {
    // 读取文件内容到一个 String
    let contents = read_to_string(filename).expect("Should have been able to read the file");
    // 将 String 转换成 Box<str>
    let boxed = contents.into_boxed_str();
    // 泄露 Box<str> 的所有权，返回一个 &'static str
    let leaked = Box::leak(boxed);
    // 返回 &'static str
    leaked
}
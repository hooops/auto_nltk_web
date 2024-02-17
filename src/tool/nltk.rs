// // 引入标准库中的文件和json模块
// use std::fs::File;
// use std::io::Read;
// use tinyjson::JsonValue;
// use std::collections::HashMap;
// use std::convert::TryInto;
// // use tracing::Value;
// use serde_json::{ Value};
// use anyhow::Result;
// use serde::{Deserialize,Serialize};
// use crate::util::md5::md5s;

// // d=['RB', 'VBP', 'WDT', 'CC', 'VBG', 'VBD', 'PRP$', 'NNS', 'VBZ',   'DT',  'JJ', 'IN', 'NNP', 'PRP', 'TO', 'WP', 'RBS', 'JJS', 'RP', 'NNPS', 'EX']

// // 定义一个结构体，用于存储json对象的属性
// #[derive(Debug, Serialize, Deserialize)]
// pub struct Info {
//     RB: Vec<String>,
//     // VBP: Vec<String>,
//     CC:Vec<String>,
//     // WDT:Vec<String>,
//     VBG:Vec<String>,
//     VBD:Vec<String>,
//     NNS:Vec<String>,
//     VBZ:Vec<String>,
//     DT:Vec<String>,
//     JJ:Vec<String>,
//     IN:Vec<String>,
//     NNP:Vec<String>,
//     PRP:Vec<String>,
//     TO:Vec<String>,
//     // WP:Vec<String>,
//     // RBS:Vec<String>,
//     JJS:Vec<String>,
//     // RP:Vec<String>,
//     // NNPS:Vec<String>,
//     // EX:Vec<String>
// }
// // d=['RB', 'VBP', 'WDT', 'CC', 'VBG', 'VBD', 'PRP$', 'NNS', 'VBZ',   'DT',  'JJ', 'IN', 'NNP', 'PRP', 'TO', 'WP', 'RBS', 'JJS', 'RP', 'NNPS', 'EX']

// fn value_to_hashmap(value: Value) -> HashMap<&'static &std::string::String, &'static &std::string::String> {
//     let map: Map<String, Value> = serde_json::from_value(value).unwrap();
//     let mut new_map = HashMap::new();

//     for (key, value) in map {
//         if let Value::String(v) = value {
//             new_map.insert(&key, &v);
//         }
//     }

//     new_map
// }

// // 定义一个函数，接受一个文件名和一个整数作为参数，返回一个Result类型
// fn read_and_parse_json(filename: &str, serdeNum:Vec<u32>) ->HashMap<&'static &std::string::String, &'static &std::string::String>{
//     // 打开文件，如果出错，返回错误

//     let mut file = File::open(filename).expect("Unable to open file");

//     // 创建一个空的字符串变量，用于存储文件内容
//     let mut content = String::new();
//     // 读取文件内容到字符串变量，如果出错，返回错误

//     file.read_to_string(&mut content).expect("Unable to read file");

//     // println!("{}", content);

//     let v: Value = serde_json::from_str(&content).expect("Unable to read file");

//     let result = value_to_hashmap(v);

//     // 打印Person结构体的内容

//     // let mut rng = SmallRng::from_rng(thread_rng()).unwrap();
//     // println!("{}", rng.gen_range(0..100));

// let vec_data=vec!["RB", "VBP", "WDT", "CC", "VBG", "VBD", "NNS", "VBZ",   "DT",  "JJ", "IN", "NNP", "PRP", "TO", "WP", "RBS", "JJS", "RP", "NNPS", "EX"];
// for (key, value) in &result {
//     println!("{}: {}", key, value);
// }
// result

//     // 返回向量作为成功的结果

// }

// // 测试函数，使用一个示例文件
// pub fn word_tag(key:&str) -> &HashMap<&'static &std::string::String, &'static &std::string::String>{

//     let url=&md5s(&md5s(&"3331".to_string())).to_string()[0..7];
//     let s =url;
//     let total_unicode_sum: u32 = s.chars().map(|c| c as u32).fold(0, |acc, code| acc+1000 + code+1000);
//     println!("Total unicode sum: {}", total_unicode_sum);
//     let num = total_unicode_sum;
//     let num_str = num.to_string();
//     let num_array: Vec<u32> = num_str.chars().map(|c| c.to_digit(10).unwrap()).collect();
//     println!("Num array: {:?}", num_array);

//     // 调用函数，传入文件名和限制
//     let mut indexNum:Vec<u32>=Vec::new();
//     indexNum.push(1);
//     indexNum.push(111);
//     let result = read_and_parse_json("src//lib_static//word_tag.json", num_array);

//      &result
//     // 根据结果，打印输出

// }

use crate::init::static_json::static_json_txt;
use serde_json;
use std::fs;
pub fn word_tag(mut serdeNum: Vec<u32>) -> String {
    let tag: Vec<&str> = vec![
        "RB", "VBP", "WDT", "CC", "VBG", "VBD", "NNS", "VBZ", "DT", "JJ", "IN", "NNP", "PRP", "TO",
        "WP", "RBS", "JJS", "RP", "NNPS", "EX",
    ];
    // d=['RB', 'VBP', 'WDT', 'CC', 'VBG', 'VBD', 'PRP$', 'NNS', 'VBZ',   'DT',  'JJ', 'IN', 'NNP', 'PRP', 'TO', 'WP', 'RBS', 'JJS', 'RP', 'NNPS', 'EX']

    // let serdeNum:Vec<u32> = vec![1, 2,39,11,23];
    //     let file_path = "/root/appx/src/lib_static/word_tag.json";
    //     // let file_path = "C:\\Users\\magib\\Desktop\\vizApp\\src\\lib_static\\word_tag.json";
    serdeNum.extend(serdeNum.clone());
    let serde_len = serdeNum.len();
    //     // 用read_to_string函数来把文件的内容读取成一个String
    //     // 如果读取失败，就打印出错误信息
    //     let json = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let json = static_json_txt();
    let value: serde_json::Value = serde_json::from_str(&json).unwrap();

    let mut word_li: Vec<String> = Vec::new();
    let mut index = 0;
    let mut indexNum = 0;
    for i in tag.into_iter().cycle() {
        let name = value.get(i);
        if name.is_none() {
            continue;
        } else {
            let v0 = match name {
                Some(n) => n,
                None => todo!(),
            };
            // println!("{:?}",v0);
            // let v1=match v0{
            //      Some(v) =>  v[index],

            // };

            let q = serdeNum.get(index as usize);

            let v2 = v0
                .get(*q.unwrap_or(&0) as usize)
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or("".to_string());

            let v3 = v2.clone();
            if v3.len() > 2 {
                word_li.push(v3);
            }

            // word_li.push(d.expect("REASON")..as_str());
        }
        // 用get方法来根据键来获取值
        index = index + 1;

        if index == serde_len {
            index = 0;
            indexNum += 1;
        }
        if indexNum > 10 {
            break;
        }
    }
    word_li.join(" ").replace('"', "")
}

// 定义一个函数，接受一个字符串引用作为参数
pub fn string_to_unicode_digits(s: &str) -> Vec<u32> {
    // 定义一个空的结果数组
    let mut result = Vec::new();

    // 遍历字符串中的每个字符
    for c in s.chars() {
        // 如果字符是一个符号，就跳过它
        if c.is_ascii_punctuation() {
            continue;
        }
        // 否则，把字符转换成u32类型，然后取低8位作为u8类型
        let num = c as u32 & 0xff;
        // 把数字添加到结果数组中

        result.push(num);
    }
    // 如果结果数组的长度是奇数，就在末尾补一个0

    // 返回结果数组
    result
}

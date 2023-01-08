use std::fs::{self, File};
use std::io::Read;

use serde_json::json;
use serde_json::value::Value;
use serde_json::Map;

/** 读取文件 */
pub fn read_file(file_name: &str) -> Map<String, Value> {
    // 打开文件
    let mut file = File::open(file_name).unwrap();
    // 创建一个 string buffer 用于存储数据
    let mut data = String::new();
    // 读取数据写入到 buffer
    file.read_to_string(&mut data).unwrap();
    // 将读取到的字符串转换成 json 文本格式 Value 类型
    let json: Value = serde_json::from_str(&data).unwrap();
    // 将 json 文本格式转换成 json 对象
    let state: Map<String, Value> = json.as_object().unwrap().clone();
    // 将这个对象返回出去
    return state;
}

/** 写入文件 */
pub fn write_to_file(file_name: &str, state: &mut Map<String, Value>) {
    // json! 这个宏用于将 json 字面量对象转换回 json 文本 Value 格式
    let new_data = json!(state);
    // 将 json 文本写入到文件中
    fs::write(file_name, new_data.to_string()).expect("unable to write to file");
}

use serde_json::{Map, Value};

pub trait Get {
    // 为这个 trait 实现一个默认的 get 方法
    fn get(&self, title: &String, state: &Map<String, Value>) {
        let item = state.get(title);
        match item {
            Some(result) => {
                println!("\n\nItem: {}", title);
                println!("Status: {} \n\n", result);
            }
            None => println!("item: {} was not find", title),
        }
    }
}

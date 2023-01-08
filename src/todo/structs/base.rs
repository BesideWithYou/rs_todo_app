pub struct Base {
    pub title: String,
    pub status: String,
}

// 为这个 Base 数据结构实现一个 new 方法，并返回一个实例化后的数据结构
impl Base {
    pub fn new(input_title: &str, input_status: &str) -> Base {
        Base {
            title: input_title.to_string(),
            status: input_status.to_string(),
        }
    }
}

mod todo;

// 必须导入了这个 trait 下面才能进行调用
use todo::structs::traits::create::Create;
use todo::todo_factory;
use todo::ItemTypes;

fn main() {
    let todo_item = todo_factory("pending", "washing");
    match todo_item.unwrap() {
        ItemTypes::Pending(item) => {
            println!(
                "This is a pending item with title: {}",
                item.super_struct.title
            );
            item.create(&item.super_struct.title);
        }
        ItemTypes::Done(item) => {
            println!(
                "This is a done item with title: {}",
                item.super_struct.title
            );
        }
    }
}

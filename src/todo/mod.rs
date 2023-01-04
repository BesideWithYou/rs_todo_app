pub mod structs;
use structs::done::Done;
use structs::pending::Pending;

pub enum ItemTypes {
    Pending(Pending),
    Done(Done),
}

pub fn todo_factory(item_type: &str, item_title: &str) -> Result<ItemTypes, &'static str> {
    match item_type {
        "pending" => {
            let pending_item = Pending::new(item_title);
            Ok(ItemTypes::Pending(pending_item))
        }
        "done" => {
            let done_item = Done::new(item_title);
            Ok(ItemTypes::Done(done_item))
        }
        _ => Err("This is not accepted!"),
    }
}

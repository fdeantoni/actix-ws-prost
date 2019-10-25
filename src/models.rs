use std::io::Cursor;

use prost::Message as Message;

// Include the `items` module, which is generated from items.proto.
pub mod items {
    include!(concat!(env!("OUT_DIR"), "/snazzy.items.rs"));
}

pub fn create_large_shirt(color: String) -> items::Shirt {
    let mut shirt = items::Shirt::default();
    shirt.color = color;
    shirt.set_size(items::shirt::Size::Large);
    shirt
}

pub fn serialize_shirt(shirt: &items::Shirt) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(shirt.encoded_len());
    // Unwrap is safe, since we have reserved sufficient capacity in the vector.
    shirt.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_shirt(buf: &[u8]) -> Result<items::Shirt, prost::DecodeError> {
    items::Shirt::decode(&mut Cursor::new(buf))
}

#[cfg(test)]
mod tests {
    use crate::models::*;

    #[test]
    fn create_shirt() {
        println!("{}", env!("OUT_DIR"));
        let shirt = create_large_shirt("white".to_string());
        println!("shirt is {:?}", &shirt);
        assert_eq!(shirt.color, "white");
    }

    #[test]
    fn serde_shirt() {
        let shirt = create_large_shirt("white".to_string());
        let serded = deserialize_shirt(&serialize_shirt(&shirt) ).expect("A shirt!");
        println!("Serded {:?}", serded);
        assert_eq!(serded, shirt);
    }
}
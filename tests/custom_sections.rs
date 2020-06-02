#[link_section = "easter_egg"]
pub static EASTER_EGG: [u8; 6] = *b"Wasmer";

#[link_section = "hello"]
pub static HELLO: [u8; 6] = *b"World!";

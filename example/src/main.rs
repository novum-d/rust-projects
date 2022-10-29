use std::ops::RangeInclusive;

fn main() {
    let some_u8_value: Option<u8> = Some(0u8); // u8: 0 ~ 255(u8::MIN ~ u8::MAX)
    let _range: RangeInclusive<u8> = u8::MIN..=u8::MAX; // u8::MIN..(u8::MAX + 1)
    match some_u8_value {
        Some(_range) => println!("some"),
        None => println!("none"),
    }
}

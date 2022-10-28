use std::ops::RangeInclusive;

fn main() {
    let some_u8_value: Option<u8> = Some(0u8); // u8: 0 ~ 255(u8::MIN ~ u8::MAX)
    let range: RangeInclusive = u8::MIN..=u8::MAX;
    match some_u8_value {
        Some(u8::MIN..=u8::MAX) => println!("some"),
        None => println!("none"),
    }
}

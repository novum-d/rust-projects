use snip::extensions::extends::IntExt;

fn main() {
    let double = IntExt::double(&5);
    println!("{}, {}", 5.double(), double);
}

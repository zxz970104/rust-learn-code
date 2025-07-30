fn main() {
    let a: u32 = 10;

    // Cast `a` into the `u64` type
    let b = a as u64;

    // You can use `_` as the target type
    // if it can be correctly inferred
    // by the compiler. For example:
    let c: u64 = a as _;

    println!("a: {}, b: {}, c: {}", a, b, c);

    // A number that's too big
    // to fit into a `u8`
    let a: u16 = 255 + 1;
    let b = a as u8;
    println!("{} as a u8 is {}", a, b);


}

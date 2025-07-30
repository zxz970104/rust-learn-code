fn main() {
    let x = 255u8;
    let y = 1u8;
    let sum1 = x.wrapping_add(y);
    println!("sum1: {}", sum1);
    assert_eq!(sum1, 0);

    let sum2 = x.saturating_add(y);
    println!("sum2: {}", sum2);
    assert_eq!(sum2, 255);
}

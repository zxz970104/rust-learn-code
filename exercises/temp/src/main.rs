pub trait IsEven {
    fn is_even(&self) -> bool;
}

#[derive(Debug)]
pub struct MyNumber(pub i32);

impl IsEven for MyNumber {
    fn is_even(&self) -> bool {
        self.0 % 2 == 0
    }
}

fn print_if_even<T: IsEven + std::fmt::Debug>(n: T) {
    if n.is_even() {
        println!("{n:?} is even");
    }
}

fn main() {
    let num = MyNumber(42);
    print_if_even(num);
}

mod person_module {
    // 结构体默认私有
    pub struct Person {
        name: String,
        age: u8,
        phone_number: String,
        address: String,
    }

    impl Person {
        // 公共的构造函数
        pub fn new(name: String, age: u8, phone_number: String, address: String) -> Person {
            Person {
                name,
                age,
                phone_number,
                address,
            }
        }

        pub fn get_name(&self) -> &String {
            &self.name
        }

        pub fn get_age(&self) -> u8 {
            self.age
        }

        pub fn set_age(&mut self, age: u8) {
            self.age = age;
        }

        pub fn get_phone_number(&self) -> &String {
            &self.phone_number
        }

        pub fn get_address(&self) -> &String {
            &self.address
        }
    }
}

fn main() {
    // 使用自定义的构造函数
    let mut p1 = person_module::Person::new(
        "John Doe".to_string(),
        30,
        "123-456-7890".to_string(),
        "1234 Elm Street".to_string(),
    );

    p1.set_age(31);

    print!("Name: {}\n", p1.get_name());
    print!("Age: {}\n", p1.get_age());
    print!("Phone Number: {}\n", p1.get_phone_number());
    print!("Address: {}\n", p1.get_address());

    let a:usize = std::mem::size_of::<person_module::Person>();
    print!("Size of Person: {}\n", a);

    let mut s = String::with_capacity(5);
    s.push_str("Hey");
    print!("String: {}\n", s);
    print!("Capacity: {}\n", s.capacity());
    print!("Length: {}\n", s.len());
    std::mem::drop(x);

}

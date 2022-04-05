struct Person {
    name: String,
    age: u32
}

impl Person {
    fn new(name: &str, age: u32) -> Self {
        Self {
            name: name.to_string(),
            age
        }
    }

    fn happy_birthday(&mut self) -> &mut Self {
        self.age = self.age + 1;
        self
    }
}

fn main() {
    let mut me = Person::new("Stefan", 39);
    me.happy_birthday().happy_birthday();
    me.happy_birthday();

    

    println!("{} is {} years old", me.name, me.age);
}
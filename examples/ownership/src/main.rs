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
}


fn main() {
    let mut a = print_and_return(vec![39, 40, 41, 42]);
    let b = a.clone();
    let mut_a = &mut a;
    edit_vec(mut_a);
    edit_vec(mut_a);
    let c = &a;
    let d = &b;
    print_vec(&b);
    print_vec(c);
    print_vec(d);
    print_vec(&a);

    let me = Person::new("Stefan", 39);
    print_person(&me);
}

fn print_person(person: &Person) {
    println!("{} is {} years old", person.name, person.age);
}

fn print_and_return(vec: Vec<i32>) -> Vec<i32> {
    println!("{:?}", vec);
    vec
}

fn edit_vec(vec: &mut Vec<i32>) {
    vec.push(43);
}

fn print_vec(vec: &Vec<i32>) {
    println!("{:?}", vec);
}
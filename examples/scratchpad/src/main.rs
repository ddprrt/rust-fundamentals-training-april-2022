struct Point {
    x: f32,
    y: f32
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self {
            x, y
        }
    }
    
    fn length(self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Person {
    name: String,
    age: u32
}

impl Person {
    // associated function
    fn new(name: &str, age: u32) -> Self {
        Self {
            name: name.to_string(),
            age
        }
    }

    // associated function, method
    fn print_me(self) {
        println!("Hi, I'm {}, and I'm {} years old", self.name, self.age);
    }
}

fn main() {
    let a_number = 1;
    let another_number = 2;

    let _ = add(a_number, another_number);

    let an_explicit_annotation = 10_u16;
    let _an_explicit_annotation = an_explicit_annotation + 11_u16;


    let a_number = 1;
    let another_number = 2;

    let _result = add(a_number, another_number);

    let an_explicit_annotation = 10_u16;
    let _an_explicit_annotation = an_explicit_annotation + 11_u16;

    let _range = 0..10;

    for i in 0..20 {
        println!("{}", match i {
            0..=10 => "Still ok",
            15 => "Halfway through",
            _ => "No idea what to do with that"
        });
    }

    let array_one = [1, 2, 3, 4, 5];
    let slice_of_array_one = &array_one[..];

    let vector_one = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let slice_of_vector_one = &vector_one[..=4];
    
    print_slice(slice_of_array_one);
    print_slice(slice_of_vector_one);

    let _collected_vector: Vec<u16> = (1..5).collect();

    print_collected_vector((1..5).collect());

    let _person = Person { name: String::from("Stefan"), age: 39 };
    let another_person = Person::new("Stefan", 39);
    another_person.print_me();

    println!("{}", Point::new(2.0, 3.0).length());

    let point = Point::new(2.0, 3.0);
    let _length = Point::length(point);
    //let _length = point.length();
}

fn print_collected_vector(vec: Vec<u16>) {
    println!("{:?}", vec);
}

fn print_slice(slice: &[i32]) {
    println!("{:?}", slice);
    println!("{}", slice.len());
}

fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[test]
fn test_my_work() {
    assert_eq!(2, 2);
}

#[test]
#[ignore]
fn test_my_work2() {
    assert_eq!(2, 2);
}

struct Individual {
    name: String,
}

enum Room {
    Occupied(Individual),
    Vacant,
}

impl Room {
    fn is_occupied(self) -> bool {
        match self {
            Room::Occupied(_) => true,
            Room::Vacant => false
        }
    }
}
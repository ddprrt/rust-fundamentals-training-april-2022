use std::fmt::Display;

#[derive(Debug, PartialEq, PartialOrd, Default)]
struct Kilometers(f64);

#[derive(Debug, Default)]
pub struct Person {
    pub name: String,
    pub age: u32
}

#[derive(PartialEq, Eq)]
struct Age(u8);

impl Kilometers {
    fn new(kilometers: f64) -> Self {
        Self(kilometers)
    }
}

impl Display for Kilometers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} km", self.0)
    }
}

impl std::ops::Add for Kilometers {
    type Output = Kilometers; // Associated Type

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.0 + rhs.0)
    }
}

impl std::ops::Add<f64> for Kilometers {
    type Output = Kilometers;

    fn add(self, rhs: f64) -> Self::Output {
        Self::new(self.0 + rhs)
    }
}

impl std::ops::Add<Kilometers> for f64 {
    type Output = Kilometers;

    fn add(self, rhs: Kilometers) -> Self::Output {
        Kilometers::new(self + rhs.0)
    }
}

impl From<f64> for Kilometers {
    fn from(x: f64) -> Self {
        Self::new(x)
    }
}

fn print_value<T: ToString>(val: &T) {
    let string_representation = val.to_string();
    println!("{}", string_representation);
}

fn main() {
    let _from_floats: Kilometers = 1000.0.into();
    let driven_kms = Kilometers::new(1000.0);
    let some_more_kms = Kilometers::new(1001.0);
    let no_kms = Kilometers::default();
    let _me = Person { name: "Stefan".to_string(), age: 39 };
    println!("{:?}", driven_kms);
    println!("{}", driven_kms);
    println!("{}", driven_kms > some_more_kms);
    println!("{}", no_kms);
    println!("{}", driven_kms + some_more_kms);
    println!("{}", 50.0 + Kilometers::new(1000.0));

    let kms = Kilometers::new(1234.0);
    print_value(&kms);

    //println!("{}", 0.1 + 0.2);
}

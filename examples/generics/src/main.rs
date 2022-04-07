struct Container<T> {
    a: T,
    b: T
}

impl<T> Container<T> {
    fn new(a: T, b: T) -> Self {
        Self {
            a, b
        }
    }
}
/*
impl Container<i32> {
    fn add(&self) -> i32 {
        self.a + self.b
    }
}

impl Container<f64> {
    fn add(&self) -> f64 {
        self.a + self.b
    }
}
 */

impl<T> Container<T>
where T: std::ops::Add + Copy {
    fn sum_up(&self) -> T::Output {
        self.a + self.b
    }
}

#[derive(Debug, PartialEq, PartialOrd, Default, Copy, Clone)]
struct Kilometers(f64);
impl Kilometers {
    fn new(kilometers: f64) -> Self {
        Self(kilometers)
    }
}
impl std::ops::Add for Kilometers {
    type Output = f64; // Associated Type

    fn add(self, rhs: Self) -> Self::Output {
        self.0 + rhs.0
    }
}

trait Print {
    fn print(&self);
}

impl<T: std::fmt::Debug> Print for T {
    fn print(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let _container = Container::new(1, 2).sum_up();
    let _container = Container::new(1.0, 2.0).sum_up();
    let _container = Container::new(1_u8, 2_u8).sum_up();
    let _container = Container::new(Kilometers::new(1.0), Kilometers::new(2.0)).sum_up();
    let _container = Container::new("a", "b");
    
    2.print();
    "Hello world".print();
    vec![1, 2, 3, 4, 5, 6, 7].print();
}

struct Project {
    price: f32
}

impl Project {
    fn new(price: f32) -> Self {
        Self { price }
    }
}

impl Billable for Project {
    fn bill(&self) -> f32 {
        self.price
    }
}

struct Maintenance {
    rate: f32,
    hours: f32
}

impl Maintenance {
    fn new(rate: f32, hours: f32) -> Self {
        Self { rate, hours }
    }

    fn rate(&self) -> f32 {
        self.rate
    }
}

impl Billable for Maintenance {
    fn bill(&self) -> f32 {
        self.rate * self.hours
    }
}

trait Billable {
    fn bill(&self) -> f32;

    fn as_cent_value(&self) -> f32 {
        self.bill() * 100.0
    }
}

fn print_price<T: Billable>(billable: &T) {
    println!("This item costs {} ct", billable.as_cent_value());
}

impl Billable for f32 {
    fn bill(&self) -> f32 {
        *self
    }

    fn as_cent_value(&self) -> f32 {
        self.bill()
    }
}

fn main() {
    let project = Project::new(5000.0);
    let maintenance = Maintenance::new(200.0, 10.0);
    maintenance.rate();
    print_price(&project);
    print_price(&maintenance);
    print_price(&100.0);
}

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self { hours, minutes }.normalize()
    }

    fn normalize(self) -> Self {
        let mut hours = (self.hours + self.minutes / 60) % 24;
        let mut minutes = self.minutes % 60;
        if minutes < 0 {
            minutes += 60;
            hours -= 1;
        }
        if hours < 0 {
            hours = (hours + 24) % 24;
        }
        Self { hours, minutes }
    }
}

fn main() {
    // Uncomment this!
    let clock = Clock::new(10, 10);
    println!("{}", clock.hours);
    println!("{}", clock.minutes);
}

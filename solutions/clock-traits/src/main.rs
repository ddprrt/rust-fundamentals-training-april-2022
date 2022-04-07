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

impl std::ops::Add for Clock {
    type Output = Clock;

    fn add(self, rhs: Self) -> Self::Output {
        Clock::new(self.hours + rhs.hours, self.minutes + rhs.minutes)
    }
}

impl std::ops::Add<i32> for Clock {
    type Output = Clock;

    fn add(self, rhs: i32) -> Self::Output {
        Clock::new(self.hours, self.minutes + rhs)
    }
}

impl std::ops::Sub for Clock {
    type Output = Clock;

    fn sub(self, rhs: Self) -> Self::Output {
        Clock::new(self.hours - rhs.hours, self.minutes - rhs.minutes)
    }
}

impl std::ops::Sub<i32> for Clock {
    type Output = Clock;

    fn sub(self, rhs: i32) -> Self::Output {
        Clock::new(self.hours, self.minutes - rhs)
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

fn main() {
    // Uncomment this!
    let clock = Clock::new(10, 10) + Clock::new(1, 50) + 4*60;
    println!("{}", clock);

    let clock = Clock::new(10, 10) + 1000;
    println!("{}", clock);

    let clock = Clock::new(10, 10) - 70;
    println!("{}", clock);
}

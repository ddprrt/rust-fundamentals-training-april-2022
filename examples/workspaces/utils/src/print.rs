pub trait Print {
    fn print(&self);
}

impl<T: std::fmt::Debug> Print for T  {
    fn print(&self) {
        println!("{:?}", self);
    }
}
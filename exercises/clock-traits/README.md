# Exercise: Clock with Traits

Take the trait example from the first day and add a couple of traits to it to make it nicer to use.

1. Instead of using `add_minutes`, it would be nice to just use the `+` operator. Implement `std::ops::Add` for adding two clocks
2. Implement `std::ops::Add` also for adding minutes in `i32`.
3. Implement `std::ops::Sub` to substract minutes in `i32` and other `Clock`s
4. To pretty print it on the command line, implement `std::fmt::Display`. Look at format strings in the slides to see how you can add padding


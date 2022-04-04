struct Clock {
    
}

impl Clock {
    
}

fn main() {
    // Uncomment this!
    //let clock = Clock::new(10, 10);
    //println!("{}", clock.hours);
    //println!("{}", clock.minutes);
}

//
// Clock Creation
//

#[test]
#[ignore]
fn test_on_the_hour() {
    let clock = Clock::new(8, 0);
    assert_eq!(clock.hours, 8);
    assert_eq!(clock.minutes, 0);
}

#[test]
#[ignore]
fn test_midnight_is_zero_hours() {
    let clock: Clock = Clock::new(24, 0);
    assert_eq!(clock.hours, 0);
}

#[test]
#[ignore]
fn test_hour_rolls_over() {
    let clock = Clock::new(25, 0);
    assert_eq!(clock.hours, 1);
}

#[test]
#[ignore]
fn test_hour_rolls_over_continuously() {
    let clock = Clock::new(100, 0);
    assert_eq!(clock.hours, 4);
}

#[test]
#[ignore]

fn test_sixty_minutes_is_next_hour() {
    let clock = Clock::new(1, 60);
    assert_eq!(clock.hours, 2);
}

#[test]
#[ignore]
fn test_minutes_roll_over() {
    let clock = Clock::new(0, 160);
    assert_eq!(clock.hours, 2);
    assert_eq!(clock.minutes, 40);
}

#[test]
#[ignore]
fn test_minutes_roll_over_continuously() {
    let clock = Clock::new(0, 1723);
    assert_eq!(clock.hours, 4);
    assert_eq!(clock.minutes, 43);
}

#[test]
#[ignore]
fn test_hours_and_minutes_roll_over() {
    let clock = Clock::new(25, 160);
    assert_eq!(clock.hours, 3);
    assert_eq!(clock.minutes, 40);
}

#[test]
#[ignore]
fn test_hours_and_minutes_roll_over_continuously() {
    let clock = Clock::new(201, 3001);
    assert_eq!(clock.hours, 11);
    assert_eq!(clock.minutes, 1);
}

#[test]
#[ignore]
fn test_hours_and_minutes_roll_over_to_exactly_midnight() {
    let clock = Clock::new(72, 8640);
    assert_eq!(clock.hours, 0);
    assert_eq!(clock.minutes, 0);
}

#[test]
#[ignore]
fn test_negative_hour() {
    let clock = Clock::new(-1, 15);
    assert_eq!(clock.hours, 23);
    assert_eq!(clock.minutes, 15);
}

#[test]
#[ignore]
fn test_negative_hour_roll_over() {
    let clock = Clock::new(-25, 0);
    assert_eq!(clock.hours, 23);
    assert_eq!(clock.minutes, 0);
}

#[test]
#[ignore]
fn test_negative_hour_roll_over_continuously() {
    let clock = Clock::new(-91, 0);
    assert_eq!(clock.hours, 5);
    assert_eq!(clock.minutes, 0);
}

#[test]
#[ignore]
fn test_negative_minutes() {
    let clock = Clock::new(1, -40);
    assert_eq!(clock.hours, 0);
    assert_eq!(clock.minutes, 20);
}

#[test]
#[ignore]
fn test_negative_minutes_roll_over() {
    let clock = Clock::new(1, -160);
    assert_eq!(clock.hours, 22);
    assert_eq!(clock.minutes, 20);
}

#[test]
#[ignore]
fn test_negative_minutes_roll_over_continuously() {
    let clock = Clock::new(1, -4820);
    assert_eq!(clock.hours, 16);
    assert_eq!(clock.minutes, 40);
}

#[test]
#[ignore]
fn test_negative_sixty_minutes_is_prev_hour() {
    let clock = Clock::new(2, -60);
    assert_eq!(clock.hours, 1);
    assert_eq!(clock.minutes, 0);
}

#[test]
#[ignore]
fn test_negative_hour_and_minutes_both_roll_over() {
    let clock = Clock::new(-25, -160);
    assert_eq!(clock.hours, 20);
    assert_eq!(clock.minutes, 20);
}

#[test]
#[ignore]
fn test_negative_hour_and_minutes_both_roll_over_continuously() {
    let clock = Clock::new(-121, -5810);
    assert_eq!(clock.hours, 22);
    assert_eq!(clock.minutes, 10);
}

#[test]
#[ignore]
fn test_zero_hour_and_negative_minutes() {
    let clock = Clock::new(0, -22);
    assert_eq!(clock.hours, 23);
    assert_eq!(clock.minutes, 38);
}

//
// Clock Math
//

#[test]
#[ignore]
fn test_add_minutes() {
    let clock = Clock::new(10, 0).add_minutes(3);
    assert_eq!(clock.hours, 10);
    assert_eq!(clock.minutes, 03);
}

#[test]
#[ignore]
fn test_add_no_minutes() {
    let clock = Clock::new(6, 41).add_minutes(0);
    assert_eq!(clock.hours, 06);
    assert_eq!(clock.minutes, 41);
}

#[test]
#[ignore]
fn test_add_to_next_hour() {
    let clock = Clock::new(0, 45).add_minutes(40);
    assert_eq!(clock.hours, 01);
    assert_eq!(clock.minutes, 25);
}

#[test]
#[ignore]
fn test_add_more_than_one_hour() {
    let clock = Clock::new(10, 0).add_minutes(61);
    assert_eq!(clock.hours, 11);
    assert_eq!(clock.minutes, 01);
}

#[test]
#[ignore]
fn test_add_more_than_two_hours_with_carry() {
    let clock = Clock::new(0, 45).add_minutes(160);
    assert_eq!(clock.hours, 03);
    assert_eq!(clock.minutes, 25);
}

#[test]
#[ignore]
fn test_add_across_midnight() {
    let clock = Clock::new(23, 59).add_minutes(2);
    assert_eq!(clock.hours, 00);
    assert_eq!(clock.minutes, 01);
}

#[test]
#[ignore]
fn test_add_more_than_one_day() {
    let clock = Clock::new(5, 32).add_minutes(1500);
    assert_eq!(clock.hours, 06);
    assert_eq!(clock.minutes, 32);
}

#[test]
#[ignore]
fn test_add_more_than_two_days() {
    let clock = Clock::new(1, 1).add_minutes(3500);
    assert_eq!(clock.hours, 11);
    assert_eq!(clock.minutes, 21);
}

#[test]
#[ignore]
fn test_subtract_minutes() {
    let clock = Clock::new(10, 3).add_minutes(-3);
    assert_eq!(clock.hours, 10);
    assert_eq!(clock.minutes, 00);
}

#[test]
#[ignore]
fn test_subtract_to_previous_hour() {
    let clock = Clock::new(10, 3).add_minutes(-30);
    assert_eq!(clock.hours, 09);
    assert_eq!(clock.minutes, 33);
}

#[test]
#[ignore]
fn test_subtract_more_than_an_hour() {
    let clock = Clock::new(10, 3).add_minutes(-70);
    assert_eq!(clock.hours, 08);
    assert_eq!(clock.minutes, 53);
}

#[test]
#[ignore]
fn test_subtract_across_midnight() {
    let clock = Clock::new(0, 3).add_minutes(-4);
    assert_eq!(clock.hours, 23);
    assert_eq!(clock.minutes, 59);
}

#[test]
#[ignore]
fn test_subtract_more_than_two_hours() {
    let clock = Clock::new(0, 0).add_minutes(-160);
    assert_eq!(clock.hours, 21);
    assert_eq!(clock.minutes, 20);
}

#[test]
#[ignore]
fn test_subtract_more_than_two_hours_with_borrow() {
    let clock = Clock::new(6, 15).add_minutes(-160);
    assert_eq!(clock.hours, 03);
    assert_eq!(clock.minutes, 35);
}

#[test]
#[ignore]
fn test_subtract_more_than_one_day() {
    let clock = Clock::new(5, 32).add_minutes(-1500);
    assert_eq!(clock.hours, 04);
    assert_eq!(clock.minutes, 32);
}

#[test]
#[ignore]
fn test_subtract_mores_than_two_days() {
    let clock = Clock::new(2, 20).add_minutes(-3000);
    assert_eq!(clock.hours, 00);
    assert_eq!(clock.minutes, 20);
}

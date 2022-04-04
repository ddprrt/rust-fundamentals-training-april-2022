pub fn nth(n: u32) -> u32 {
    todo!("Implement this method")
}

fn main() {
    println!("{}", nth(20));
}

#[test]
fn test_first_prime() {
    assert_eq!(nth(0), 2);
}

#[test]
#[ignore]
fn test_second_prime() {
    assert_eq!(nth(1), 3);
}

#[test]
#[ignore]
fn test_sixth_prime() {
    assert_eq!(nth(5), 13);
}

#[test]
#[ignore]
fn test_big_prime() {
    assert_eq!(nth(10_000), 104_743);
}

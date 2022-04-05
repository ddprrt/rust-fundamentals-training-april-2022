fn is_prime(x: u32) -> bool {
    for i in 2..x {
        if x % i == 0 {
            return false;
        }
    }
    true
}

pub fn nth(n: u32) -> u32 {
    let mut count = 0;
    let mut num = 2;
    loop {
        if count == n {
            break;
        };

        num = num + 1;

        if is_prime(num) {
            count = count + 1;
        }
    }
    num
}

fn main() {
    println!("{}", nth(20));
}

#[test]
fn test_first_prime() {
    assert_eq!(nth(0), 2);
}

#[test]
fn test_second_prime() {
    assert_eq!(nth(1), 3);
}

#[test]
fn test_sixth_prime() {
    assert_eq!(nth(5), 13);
}

#[test]
fn test_big_prime() {
    assert_eq!(nth(10_000), 104_743);
}

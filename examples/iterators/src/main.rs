struct Fibonacci {
    curr: u32,
    next: u32
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(result) = self.curr.checked_add(self.next) {
            self.curr = self.next;
            self.next = result;
            Some(self.curr)
        } else {
            None
        }
    }
}

fn main() {
    /*let vec = vec![1, 2, 3, 4];

    for i in vec.iter() {
        println!("{}", i);
    }

    println!("{:?}", vec);*/

    for i in Fibonacci::default().into_iter() {
        println!("{}", i);
    }  

    let mut fib = Fibonacci::default().into_iter();

    loop {
        let result = fib.next();
        match result {
            Some(result) => {
                println!("{}", result);
                if result == 5 {
                    break;
                }
            },
            None => {
                break;
            }
        };
    }

    let mut fib = Fibonacci::default().into_iter();

    while let Some(result) = fib.next() {
        println!("{}", result);
        if result == 5 {
            break;
        }
        
    }

    for result in Fibonacci::default() {
        println!("{}", result);
        if result == 5 {
            break;
        }
    }

    for i in Fibonacci::default().take(10) {
        println!("{}", i);
    }

    println!("{}", Fibonacci::default().nth(0).unwrap());
    
    let _collection = Vec::from_iter(Fibonacci::default().take(5));
}

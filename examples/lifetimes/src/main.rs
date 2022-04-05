fn get_biggest<'a>(n: &'a i32, m: &'a i32) -> &'a i32 {
    if n > m {
        n
    } else {
        m
    }
}

struct Highscores {
    scores: Vec<u32>
}

impl Highscores {
    fn new(scores: Vec<u32>) -> Self {
        Self {
            scores
        }
    }

    fn get_scores(&self) -> &[u32] {
        &self.scores
    }

    fn add_value(&mut self, val: u32) -> &Self {
        self.scores.push(val);
        self
    }
}

fn main() {
    let a = 5;
    let biggest;
    
    let b = 6;
    biggest = get_biggest(&a, &b);
    println!("{}", biggest);

    //let high_scores; 
    /*if biggest < &3 {
        let scores = [10, 30, 90, 30, 100, 20, 10, 0, 30, 40, 40, 70, 70];
        high_scores = Highscores::new(&scores);
        println!("{:?}", high_scores.scores);
    } else {
        let scores = [10, 30, 90, 30, 100, 20, 10];
        high_scores = Highscores::new(&scores);
        println!("{:?}", high_scores.scores);
    }*/

    let scores = vec![10, 30, 90, 30, 100, 20, 10];
    let mut _high_scores = Highscores::new(scores);
    //high_scores.add_value(3).add_value(2);
    //println!("{:?}", high_scores.get_scores());
    
}

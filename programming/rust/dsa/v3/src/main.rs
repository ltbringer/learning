pub struct Stepper {
    curr: i32,
    step: i32,
    max: i32
}

impl Iterator for Stepper {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.curr > self.max {
            None
        } else {
            let ret = self.curr;
            self.curr += self.step;
            Some(ret)
        }
    }
}

fn main() {
    let mut n = 0;
    let mut st1 = Stepper { curr: 1, step: 1, max: 10 };
    let mut st2 = Stepper { curr: 1, step: 1, max: 10 };
    loop {
        match st1.next() {
            Some(i) => println!("loop st: {}", i),
            None => break
        }
    }

    while let Some(i) = st2.next() {
        println!("while st: {}", i);
    }

    while n < 10 {
        n += 1;
        println!("while, {}!", n);
    }

    let it = Stepper{curr: 1, step: 1, max: 10};
    for i in it {
        println!("for st: {}", i);
    }

    for n in 1..11 {
        println!("for, {}!", n);
    }

    println!("All done");
}

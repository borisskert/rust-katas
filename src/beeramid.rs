/// https://www.codewars.com/kata/51e04f6b544cf3f6550000c1/train/rust
pub fn beeramid(bonus: i32, price: f32) -> usize {
    let x = Squares::new()
        .scan(Cans::empty().with_price(price), |cans, x| {
            let new_cans = cans.add(x);
            
            if new_cans.bonus > bonus as f32 {
                None
            } else {
                *cans = new_cans;
                Some(())
            }
        });

    x.count()
}

struct Cans {
    count: i32,
    price: f32,
    bonus: f32,
}

impl Cans {
    fn empty() -> Cans {
        Cans {
            count: 0,
            price: 0.0,
            bonus: 0.0,
        }
    }

    fn with_price(&self, price: f32) -> Cans {
        Cans {
            count: self.count,
            price,
            bonus: self.count as f32 * price,
        }
    }

    fn add(&self, count: i32) -> Cans {
        let new_count = self.count + count;
        
        Cans {
            count: new_count,
            price: self.price,
            bonus: new_count as f32 * self.price,
        }
    }
}

struct Squares {
    current: i32,
}

impl Squares {
    fn new() -> Self {
        Self {
            current: 1,
        }
    }
}

impl Iterator for Squares {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current * self.current;
        self.current += 1;
        
        Some(result)
    }
}

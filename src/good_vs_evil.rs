/// https://www.codewars.com/kata/52761ee4cffbc69732000738/train/rust
pub fn good_vs_evil(good: &str, evil: &str) -> String {
    let good = Good::from_str(good);
    let evil = Evil::from_str(evil);

    let good_power = good.power();
    let evil_power = evil.power();

    if good_power > evil_power {
        return String::from("Battle Result: Good triumphs over Evil");
    }

    if good_power < evil_power {
        return String::from("Battle Result: Evil eradicates all trace of Good");
    }

    String::from("Battle Result: No victor on this battle field")
}

struct Good {
    hobbits: i32,
    men: i32,
    elves: i32,
    dwarves: i32,
    eagles: i32,
    wizards: i32,
}

struct Evil {
    orcs: i32,
    men: i32,
    wargs: i32,
    goblins: i32,
    uruk_hai: i32,
    trolls: i32,
    wizards: i32,
}

impl Good {
    fn from_str(s: &str) -> Good {
        let mut parts = s.split_whitespace()
            .map(|part| part.parse::<i32>().unwrap());

        Good {
            hobbits: parts.next().unwrap(),
            men: parts.next().unwrap(),
            elves: parts.next().unwrap(),
            dwarves: parts.next().unwrap(),
            eagles: parts.next().unwrap(),
            wizards: parts.next().unwrap(),
        }
    }

    fn power(&self) -> i32 {
        self.hobbits +
            self.men * 2 +
            self.elves * 3 +
            self.dwarves * 3 +
            self.eagles * 4 +
            self.wizards * 10
    }
}

impl Evil {
    fn from_str(s: &str) -> Evil {
        let mut parts = s.split_whitespace()
            .map(|part| part.parse::<i32>().unwrap());

        Evil {
            orcs: parts.next().unwrap(),
            men: parts.next().unwrap(),
            wargs: parts.next().unwrap(),
            goblins: parts.next().unwrap(),
            uruk_hai: parts.next().unwrap(),
            trolls: parts.next().unwrap(),
            wizards: parts.next().unwrap(),
        }
    }

    fn power(&self) -> i32 {
        self.orcs +
            self.men * 2 +
            self.wargs * 2 +
            self.goblins * 2 +
            self.uruk_hai * 3 +
            self.trolls * 5 +
            self.wizards * 10
    }
}

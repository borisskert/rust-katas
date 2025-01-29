/// https://www.codewars.com/kata/5270d0d18625160ada0000e4/train/rust
pub fn score(dice: [u8; 5]) -> u32 {
    let rules = [
        rule(1, 1000, 100),
        rule(6, 600, 0),
        rule(5, 500, 50),
        rule(4, 400, 0),
        rule(3, 300, 0),
        rule(2, 200, 0),
    ];

    rules.iter().map(|r| r(dice)).sum()
}

fn rule(search_dice: u8, three: u32, one: u32) -> impl Fn([u8; 5]) -> u32 {
    move |dice: [u8; 5]| {
        let count = dice.iter().filter(|&&d| d == search_dice).count();

        if count >= 3 {
            three + (count as u32 - 3) * one
        } else {
            count as u32 * one
        }
    }
}

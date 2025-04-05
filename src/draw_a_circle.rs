// https://www.codewars.com/kata/59c804d923dacc6c41000004/train/rust
pub fn circle(radius: i32) -> String {
    Circle::new(radius).draw()
}

struct Circle {
    radius: f32,
    x: f32,
    y: f32,
}

impl Circle {
    fn new(radius: i32) -> Circle {
        Circle {
            radius: radius as f32,
            x: (radius as f32 * 2.0 - 1.0) / 2.0,
            y: (radius as f32 * 2.0 - 1.0) / 2.0,
        }
    }

    fn contains(&self, x_a: i32, y_a: i32, x_b: i32, y_b: i32) -> bool {
        let dx_a = x_a as f32 - self.x;
        let dy_a = y_a as f32 - self.y;
        let distance_sq_a = dx_a * dx_a + dy_a * dy_a;

        let dx_b = x_b as f32 - self.x;
        let dy_b = y_b as f32 - self.y;
        let distance_sq_b = dx_b * dx_b + dy_b * dy_b;

        let avg_distance_sq = (distance_sq_a + distance_sq_b) / 2.0;
        let radius_sq = self.radius * self.radius;

        avg_distance_sq <= radius_sq
    }

    fn draw(&self) -> String {
        if self.radius < 0.0 {
            return "".to_string();
        }

        if self.radius == 0.0 {
            return "\n".to_string();
        }

        let lines = (self.radius as i32 * 2) - 1;

        (0..lines)
            .map(|y| {
                (0..lines)
                    .map(|x| {
                        if self.contains(x, x + 1, y, y + 1) {
                            '█'
                        } else {
                            ' '
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
            + "\n"
    }
}

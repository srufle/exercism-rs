// Inspired By
// https://exercism.org/tracks/rust/exercises/bowling/solutions/potatosalad
#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}
#[derive(Debug, Default)]
pub struct BowlingGame {
    throws: Vec<u16>,
    second_throw: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 || (self.second_throw && pins + self.throws.last().unwrap() > 10) {
            // if pins > strike  || combined first and second frame > spare
            Err(Error::NotEnoughPinsLeft)
        } else if self.score().is_some() {
            // returning Some indicates we scored all frames
            Err(Error::GameComplete)
        } else {
            // push pins on vec, and set frame flag
            self.throws.push(pins);
            self.second_throw = if pins != 10 {
                // toggle bool
                !self.second_throw
            } else {
                // strike, so you are not on a second frame
                false
            };
            Ok(())
        }
    }

    pub fn score(&self) -> Option<u16> {
        let mut total = 0;
        let mut frame = 0;
        let throws = &self.throws;
        dbg!(throws);
        for _ in 0..10 {
            // for each throw
            // get the first and second value
            if let (Some(&first), Some(&second)) = (throws.get(frame), throws.get(frame + 1)) {
                // Keep running total
                total += dbg!(first + second);
                if first == 10 || first + second == 10 {
                    // strike or spare, add the third throw
                    if let Some(&third) = throws.get(frame + 2) {
                        total += third;
                    } else {
                        dbg!("1: else strike or spare None");
                        return None;
                    }
                }
                // if strike move 1 frame, else move 2
                frame += if first == 10 { 1 } else { 2 };
                dbg!(frame, total);
            } else {
                dbg!("2: else first and second throws None");
                return None;
            }
        }
        dbg!(total);
        Some(total)
    }
}

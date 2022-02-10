// Inspired By
// https://exercism.org/tracks/rust/exercises/bowling/solutions/potatosalad
#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}
#[derive(Debug, Default)]
pub struct BowlingGame {
    frames: Vec<u16>,
    second: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 || (self.second && pins + self.frames.last().unwrap() > 10) {
            // if pins > strike  || combined first and second frame > spare
            Err(Error::NotEnoughPinsLeft)
        } else if self.score().is_some() {
            // returning Some indicates we scored all frames
            Err(Error::GameComplete)
        } else {
            // push pins on vec, and set frame flag
            self.frames.push(pins);
            self.second = if pins != 10 { !self.second } else { false };
            Ok(())
        }
    }

    pub fn score(&self) -> Option<u16> {
        let mut total = 0;
        let mut frame = 0;
        let throws = &self.frames;
        for _ in 0..10 {
            if let (Some(&first), Some(&second)) = (throws.get(frame), throws.get(frame + 1)) {
                total += first + second;
                if first == 10 || first + second == 10 {
                    if let Some(&third) = throws.get(frame + 2) {
                        total += third;
                    } else {
                        return None;
                    }
                }
                frame += if first == 10 { 1 } else { 2 };
            } else {
                return None;
            }
        }
        Some(total)
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, PartialEq)]
pub enum Frame {
    Strike(u16),
    Spare(u16),
    OpenFrame(u16),
}

#[derive(Debug)]
pub struct BowlingGame {
    frames: Vec<Frame>,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame { frames: vec![] }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        let frames = &mut self.frames;
        let frame = frames.pop();
        let frame = match frame {
            None => {
                if pins == 10 {
                    Frame::Strike(10)
                } else {
                    Frame::OpenFrame(pins)
                }
            }
            Some(Frame::Strike(p)) => {
                frames.push(Frame::Strike(p));
                if pins == 10 {
                    Frame::Strike(10)
                } else {
                    Frame::OpenFrame(pins)
                }
            }
            Some(Frame::OpenFrame(p)) => {
                if (p + pins) > 10 {
                    return Err(Error::NotEnoughPinsLeft);
                } else if (p + pins) == 10 {
                    Frame::Spare(10)
                } else {
                    frames.push(Frame::OpenFrame(p));
                    Frame::OpenFrame(pins)
                }
            }
            Some(Frame::Spare(p)) => {
                frames.push(Frame::Spare(p));
                if pins == 10 {
                    Frame::Strike(10)
                } else {
                    Frame::OpenFrame(pins)
                }
            }
        };
        if frames.len() == 20 {
            let last_frame = &frames[frames.len() - 1];
            if last_frame != &Frame::Strike(10) || last_frame != &Frame::Spare(10) {
                return Err(Error::GameComplete);
            }
        }
        frames.push(frame);
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        dbg!(self.frames.len());

        let mut frames_played = 0;
        let mut total_score: u16 = 0;
        let frames_as_slice = &self.frames.as_slice();

        for window in frames_as_slice.windows(3) {
            dbg!(window);
            for frame in window {
                let score = match frame {
                    Frame::Strike(s) => s,
                    Frame::Spare(s) => s,
                    Frame::OpenFrame(s) => s,
                };
                total_score += score;
            }
            frames_played += 1;
        }
        dbg!(frames_played);
        if frames_played / 2 < 10 {
            return None;
        }
        Some(total_score)
    }
}

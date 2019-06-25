type Color = i32;
use crate::color::*;

pub enum Wl {
  Win,
  Lose,
  Tie
}

#[derive(Clone)]
pub enum Move {
  Mv (i32, i32),
  Pass,
  GiveUp
}

pub fn string_of_move(m: Move) -> String {
  match m {
    Move::Pass => {
      "PASS".to_string()
    }
    Move::GiveUp => {
      "GIVEUP".to_string()
    }
    Move::Mv(i, j) => {
      let ci = (i + ('A' as i32) - 1) as u8 as char;
      let cj = (j + ('1' as i32) - 1) as u8 as char;
      ci.to_string() + cj.to_string().as_str()
    }
  }
}

pub enum Command {
  Open (String),
  End (Wl, i32, i32, String),
  Move (Move),
  Start (Color, String, i32),
  Ack (i32),
  Bye (Vec<(String, (i32, i32, i32))>),
  Empty
}
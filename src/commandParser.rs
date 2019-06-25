use crate::color::*;
use crate::command::*;
use crate::commandLexer::*;

pub fn parse(tokens: &mut Vec<Token>) -> Command {
  if tokens[0] == Token::OPEN {
    if let Token::STR(s) = &tokens[1] {
      let t = (*s).clone();
      Command::Open(t)
    } else {
      panic!("Invalid Command");
    }
  } else if tokens[0] == Token::END {
    if let (Token::INT(m), Token::INT(n), Token::STR(s)) = (&tokens[2], &tokens[3], &tokens[4]) {
      let t = (*s).clone();
      match tokens[1] {
        Token::WIN  => Command::End(Wl::Win, *m, *n, t),
        Token::LOSE => Command::End(Wl::Lose, *m, *n, t),
        Token::TIE  => Command::End(Wl::Tie, *m, *n, t),
        _           => panic!("Invalid Command")
      }
    } else {
      panic!("Invalid Command");
    }
  } else if tokens[0] == Token::MOVE {
    if let Token::STR(s) = &tokens[1] {
      let t = (*s).clone();
      if t == String::from("PASS") {
        Command::Move(Move::Pass)
      } else if t == String::from("GIVEUP") {
        Command::Move(Move::GiveUp)
      } else if t.len() == 2 {
        let s0 = t.chars().nth(0).unwrap();
        let s1 = t.chars().nth(1).unwrap();

        if s0 >= 'A' && s0 <= 'H' && s1 >= '1' && s1 <= '8' {
          Command::Move(Move::Mv((s0 as i32) - ('A' as i32) + 1, 
            (s1 as i32) - ('1' as i32) + 1))
        } else {
          panic!("Invalid Command");
        }
      } else {
        panic!("Invalid Command");
      }
    } else {
      panic!("Invalid Command");
    }
  } else if tokens[0] == Token::START {
    if let (Token::STR(s), Token::INT(n)) = (&tokens[2], &tokens[3]) {
      let t = (*s).clone();
      match tokens[1] {
        Token::WHITE => Command::Start(white, t, *n),
        Token::BLACK => Command::Start(black, t, *n),
        _            => panic!("Invalid Command")
      }
    } else {
      panic!("Invalid Command")
    }
  } else if tokens[0] == Token::ACK {
    if let Token::INT(n) = &tokens[1] {
      Command::Ack(*n)
    } else {
      panic!("Invalid Command")
    }
  } else if tokens[0] == Token::BYE {
    tokens.remove(0);
    tokens.pop();
    Command::Bye(render_scores(tokens))
  } else if tokens[0] == Token::NL || tokens[0] == Token::EOF {
    Command::Empty
  } else {
    panic!("Invalid Command");
  }
}

fn render_scores(tokens: &mut Vec<Token>) -> Vec<(String, (i32, i32, i32))> {
  let mut score_v = Vec::new();

  while !tokens.is_empty() {
    if let (Token::STR(s), Token::INT(n1), Token::INT(n2), Token::INT(n3)) 
      = (&tokens[0], &tokens[1], &tokens[2], &tokens[3]) 
    {
      let t = (*s).clone();
      score_v.push((t, (*n1, *n2, *n3)));
      for _ in 0..4 {
        tokens.remove(0);
      }
    } else {
      panic!("Invalid Command");
    }
  }

  score_v
}
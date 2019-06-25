use std::process;
use regex::Regex;

pub enum Token {
  NL,
  Int(i32),
  OPEN,
  END,
  MOVE,
  START,
  ACK,
  BYE,
  WIN,
  LOSE,
  TIE,
  WHITE,
  BLACK,
  STR(String),
  EOF
}

pub fn tokenize(input: &mut String, tokens: &mut Vec<Token>) {
  let p = input;
  let re = Regex::new(r"^-?\d+").unwrap();
  let mut tmp_str = String::from("");

  while !(p.is_empty()) {
    if let Some(_) = re.captures(p.as_str()) {
      tokens.push(Token::Int(strtol(p)));
    } else {
      if p.starts_with(" ") || p.starts_with("\t") {
        remove_times(p, 1);
      } else if p.starts_with("\n") {
        tokens.push(Token::NL);
        remove_times(p, 1);
      } else if p.starts_with("OPEN") {
        tokens.push(Token::OPEN);
        remove_times(p, 4);
      } else if p.starts_with("END") {
        tokens.push(Token::END);
        remove_times(p, 3);
      } else if p.starts_with("MOVE") {
        tokens.push(Token::MOVE);
        remove_times(p, 4);
      } else if p.starts_with("START") {
        tokens.push(Token::START);
        remove_times(p, 5);
      } else if p.starts_with("ACK") {
        tokens.push(Token::ACK);
        remove_times(p, 3);
      } else if p.starts_with("BYE") {
        tokens.push(Token::BYE);
        remove_times(p, 3);
      } else if p.starts_with("WIN") {
        tokens.push(Token::WIN);
        remove_times(p, 3);
      } else if p.starts_with("LOSE") {
        tokens.push(Token::LOSE);
        remove_times(p, 4);
      } else if p.starts_with("TIE") {
        tokens.push(Token::TIE);
        remove_times(p, 3);
      } else if p.starts_with("WHITE") {
        tokens.push(Token::WHITE);
        remove_times(p, 5);
      } else if p.starts_with("BLACK") {
        tokens.push(Token::BLACK);
        remove_times(p, 5);
      } else {
        match p.chars().nth(0).unwrap() {
          ' ' | '\t' | '\n' | '\r' => {
            let s = tmp_str.clone();
            tokens.push(Token::STR(s));
            tmp_str = String::from("");
          }
          c => {
            tmp_str += &c.to_string();
          }
        }
        remove_times(p, 1);
      }
    }
  }
  tokens.push(Token::EOF);
}

fn remove_times(s: &mut String, n: usize) {
  for _ in 0..n {
    if s.is_empty() {
      eprintln!("空文字を削除しようとしています");
      process::exit(1);
    } else {
      s.remove(0);
    }
  }
}

fn strtol(s: &mut String) -> i32 {
  let mut ans = 0;
  let mut index = 0;

  let mut is_negative = false;

  if &(s.as_str())[..1] == "-" {
    is_negative = true;
    remove_times(s, 1);
  }

  for c in s.chars() {
    if let Some(n) = c.to_digit(10) {
      ans = ans * 10 + (n as i32);
    } else {
      break;
    }
    index += 1;
  }

  for _ in 0..index {
    remove_times(s, 1);
  }
  
  if is_negative {
    ans * (-1)
  } else {
    ans
  }
}
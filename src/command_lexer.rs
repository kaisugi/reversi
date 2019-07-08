use regex::Regex;

#[derive(PartialEq, Debug)]
pub enum Token {
  NL,
  INT(i32),
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
  let re2 = Regex::new(r"^[^ \t\n\r]+").unwrap();

  while !(p.is_empty()) {
    if let Some(cap) = re.captures(p.as_str()) {
      let res = (&cap[0]).to_string();
      let n = res.len();
      let m: i32 = res.parse().unwrap();
      tokens.push(Token::INT(m));
      remove_times(p, n);
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
        if let Some(cap) = re2.captures(p.as_str()) {
          let res = (&cap[0]).to_string();
          let n = res.len();
          tokens.push(Token::STR(res));
          remove_times(p, n);
        } else {
          remove_times(p, 1);
        }
      }
    }
  }
  tokens.push(Token::EOF);
}

fn remove_times(s: &mut String, n: usize) {
  let s_tmp = &s[n..];
  *s = s_tmp.to_string();
}



#[test]
fn check_tokenize() {
  let mut input = "OPEN Anon.".to_string();
  let mut tokens = Vec::new();
  tokenize(&mut input, &mut tokens);
  assert_eq!(tokens, vec![Token::OPEN, Token::STR("Anon.".to_string()), Token::EOF]);

  input = "OPEN 星宮いちご".to_string();
  tokens = Vec::new();
  tokenize(&mut input, &mut tokens);
  assert_eq!(tokens, vec![Token::OPEN, Token::STR("星宮いちご".to_string()), Token::EOF]);

  input = "START BLACK Anon. 600000".to_string();
  tokens = Vec::new();
  tokenize(&mut input, &mut tokens);
  assert_eq!(tokens, vec![Token::START, Token::BLACK, Token::STR("Anon.".to_string()), Token::INT(600000), Token::EOF]);

    input = "START BLACK 霧矢あおい 600000".to_string();
  tokens = Vec::new();
  tokenize(&mut input, &mut tokens);
  assert_eq!(tokens, vec![Token::START, Token::BLACK, Token::STR("霧矢あおい".to_string()), Token::INT(600000), Token::EOF]);

  input = "MOVE D3".to_string();
  tokens = Vec::new();
  tokenize(&mut input, &mut tokens);
  assert_eq!(tokens, vec![Token::MOVE, Token::STR("D3".to_string()), Token::EOF]);

  input = "END LOSE 29 35 DOUBLE_PASS".to_string();
  tokens = Vec::new();
  tokenize(&mut input, &mut tokens);
  assert_eq!(tokens, vec![Token::END, Token::LOSE, Token::INT(29), Token::INT(35), Token::STR("DOUBLE_PASS".to_string()), Token::EOF]);

  input = "ACK 600000".to_string();
  tokens = Vec::new();
  tokenize(&mut input, &mut tokens);
  assert_eq!(tokens, vec![Token::ACK, Token::INT(600000), Token::EOF]);

  input = "BYE playerA -4 0 4 playerB 4 4 0".to_string();
  tokens = Vec::new();
  tokenize(&mut input, &mut tokens);
  assert_eq!(tokens, vec![Token::BYE, Token::STR("playerA".to_string()), Token::INT(-4), Token::INT(0), Token::INT(4), Token::STR("playerB".to_string()), Token::INT(4), Token::INT(4), Token::INT(0), Token::EOF]);

  input = "".to_string();
  tokens = Vec::new();
  tokenize(&mut input, &mut tokens);
  assert_eq!(tokens, vec![Token::EOF]);
}
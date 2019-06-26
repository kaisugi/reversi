extern crate regex;
extern crate clap;

mod color;
mod command;
mod command_lexer;
mod command_parser;
mod play;

use clap::{Arg, App};
use std::io::{BufWriter, Write};
use std::io::{BufReader, BufRead};
use std::net::{ToSocketAddrs, TcpStream, SocketAddr, Ipv4Addr};
use command::*;
use command_lexer::*;
use command_parser::*;

fn input_command (stream: &TcpStream) -> Command {
  let report_recv = |s: String| println!("Received: {}", s);

  let mut reader = BufReader::new(stream);
  let mut msg = String::new();
  reader.read_line(&mut msg).expect("RECEIVE FAILURE!!!");
  report_recv(msg);

  let mut tokens: Vec<Token> = Vec::new();
  tokenize(&mut msg, &mut tokens);
  parse(&mut tokens)
}

fn input_command_multi (stream: &TcpStream) -> Command {
  match input_command(stream) {
    Command::Empty => input_command_multi(stream),
    r              => r
  }
}

fn output_command(stream: &TcpStream, command: Command) {
  let report_sent = |s: String| println!("Sent: {}", s);

  match command {
    Command::Move(mv) => {
      let msg = format!("MOVE {}", string_of_move(mv));
      let mut writer = BufWriter::new(stream);
      writer.write(msg.as_bytes()).expect("SEND FAILURE!!!");
      writer.flush().unwrap();
      report_sent(msg);
    }
    Command::Open(s) => {
      let msg = format!("OPEN {}", s);
      let mut writer = BufWriter::new(stream);
      writer.write(msg.as_bytes()).expect("SEND FAILURE!!!");
      writer.flush().unwrap();
      report_sent(msg);
    }
    _ => {
      panic!("Client cannot not send the commands other than MOVE and Open");
    }
  }
}

#[derive(Clone)]
enum OpMove {
  PMove (Move),
  OMove (Move)
}

fn string_of_opmove (m: OpMove) -> String {
  match m {
    OpMove::PMove (mv) => format!("+{}", string_of_move(mv)),
    OpMove::OMove (mv) => format!("-{}", string_of_move(mv))
  }
}

type Hist = Vec<OpMove>;

fn string_of_hist (x: &Hist) -> String {
  let mut s_hist = String::from("");

  for i in x {
    let j = i.clone();
    s_hist = format!("{}{} ", s_hist, string_of_opmove(j));
  }
  s_hist
}

fn print_hist (x: &Hist) {
  println!("{}", string_of_hist(x));
}

fn string_of_scores (scores: Vec<(String, (i32, i32, i32))>) {
  let mut maxlen = 0;
  for (a, _) in &scores {
    if (*a).len() > maxlen {
      maxlen = (*a).len();
    }
  }

  let mut maxslen = 0;
  for (_, (s,_,_)) in &scores {
    let string_s = format!("{}", *s);
    if string_s.len() > maxslen {
      maxslen = string_s.len();
    }
  }

  let mut ans = String::from("");
  for (a, (s,w,l)) in &scores {
    ans = format!("{}:{}") // 同じ文字の繰り返しはどうやる？
  }
}

fn print_scores (scores: Vec<(String, (i32, i32, i32))>) {
  print!("{}", string_of_scores(scores));
}

/**
 * wait_start: state = 0
 * my_move   : state = 1 
 * op_move   : state = 2
 * proc_end  : state = 3
 */


fn main() {
  let matches = App::new("Reversi")
                  .arg(Arg::with_name("host name")
                    .short("H")
                    .help("host name (default = local host)")
                    .takes_value(true))
                  .arg(Arg::with_name("port number")
                    .short("p")
                    .help("port number (default = 3000)")
                    .takes_value(true))
                  .arg(Arg::with_name("player name")
                    .help("player name (default = Anon.)")
                    .short("n")
                    .takes_value(true))
                  .arg(Arg::with_name("verbose mode")
                    .help("verbose mode")
                    .short("v")
                    .takes_value(true))
                  .get_matches();          

  let opt_verbose: bool = match matches.value_of("verbose mode") {
    Some(s) => s.parse().unwrap(),
    None    => false
  };
  let opt_player_name: String = match matches.value_of("player name") {
    Some(s) => s.parse().unwrap(),
    None    => String::from("Anon.")
  };
  let opt_port: i32 = match matches.value_of("port number") {
    Some(s) => s.parse().unwrap(),
    None    => 3000
  };
  let opt_host: String = match matches.value_of("host name") {
    Some(s) => s.parse().unwrap(),
    None    => String::from("localhost")
  };

  let host_and_port = format!("{}:{}", opt_host, opt_port);
  let mut addrs = host_and_port.to_socket_addrs().unwrap();
  
  // IPv6のアドレスが混入すると上手く接続できないので, 絞る
  if let Some(addr) = addrs.find(|x| (*x).is_ipv4()) {
    match TcpStream::connect(addr) {
      Err(_) => {
        println!("Connection NG.");
      }
      Ok(stream) => {
        println!("Connection Ok.");
  
        output_command(&stream, Command::Open(opt_player_name));
      }
    }
  } else {
    eprintln!("Invalid Host:Port Number");
  }

}
extern crate regex;
extern crate clap;

mod color;
mod command;
mod commandLexer;
mod commandParser;
mod play;

use clap::{Arg, App};
use std::io::{BufWriter, Write};
use std::net::{ToSocketAddrs, TcpStream};
use command::*;

fn output_command(stream: &TcpStream, command: Command) {
  let report_sent = |s: String| println!("Sent: {}", s);

  match command {
    Command::Move(mv) => {
      let msg = format!("MOVE {}", string_of_move(mv));
      let mut writer = BufWriter::new(stream);
      writer.write(msg.as_bytes()).unwrap();
      writer.flush().unwrap();
      report_sent(msg);
    }
    Command::Open(s) => {
      let msg = format!("OPEN {}", s);
      let mut writer = BufWriter::new(stream);
      writer.write(msg.as_bytes()).unwrap();
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
  let mut addr = addrs.next().unwrap();
  println!("Connecting to {} {}.", opt_host, opt_port);

  match TcpStream::connect(addr) {
    Err(_) => {
      println!("Connection NG.");
    }
    Ok(stream) => {
      println!("Connection Ok.");

      output_command(&stream, Command::Open(opt_player_name));
    }
  }

}
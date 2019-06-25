extern crate regex;
extern crate clap;

mod color;
mod command;
mod commandLexer;
mod commandParser;

use clap::{Arg, App};
use std::env;
use std::net::{ToSocketAddrs, TcpStream};
use color::Color;

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


  let mut addrs = format!("{}:{}", opt_host, opt_port).to_socket_addrs().unwrap();
  let mut addr = addrs.next().unwrap();
  println!("Connecting to {} {}.", opt_host, opt_port);

  if let Ok(stream) = TcpStream::connect(addr) {
    println!("Connection Ok.")
  }
}
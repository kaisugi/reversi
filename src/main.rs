extern crate clap;
extern crate rand;
extern crate regex;

mod color;
mod command;
mod command_lexer;
mod command_parser;
mod play;

use clap::{App, Arg};
use std::io::{BufRead, BufReader};
use std::io::{BufWriter, Write};
use std::net::{TcpStream, ToSocketAddrs};

use color::*;
use command::*;
use command_lexer::*;
use command_parser::*;
use play::*;

fn input_command(reader: &mut BufReader<&TcpStream>) -> Command {
    let report_recv = |s: &String| print!("Received: {}", *s);

    let mut msg = String::new();
    reader.read_line(&mut msg).expect("RECEIVE FAILURE!!!");
    report_recv(&msg);

    let mut tokens: Vec<Token> = Vec::new();
    tokenize(&mut msg, &mut tokens);
    parse(&mut tokens)
}

fn input_command_multi(reader: &mut BufReader<&TcpStream>) -> Command {
    match input_command(reader) {
        Command::Empty => input_command_multi(reader),
        r => r,
    }
}

fn output_command(writer: &mut BufWriter<&TcpStream>, command: Command) {
    let report_sent = |s: String| print!("Sent: {}", s);

    match command {
        Command::Move(mv) => {
            let msg = format!("MOVE {}\n", string_of_move(mv));
            writer.write(msg.as_bytes()).expect("SEND FAILURE!!!");
            writer.flush().unwrap();
            report_sent(msg);
        }
        Command::Open(s) => {
            let msg = format!("OPEN {}\n", s);
            writer.write(msg.as_bytes()).expect("SEND FAILURE!!!");
            writer.flush().unwrap();
            report_sent(msg);
        }
        _ => {
            panic!("Client cannot send the commands other than MOVE and Open");
        }
    }
}

#[derive(Clone)]
enum OpMove {
    PMove(Move),
    OMove(Move),
}

fn string_of_opmove(m: OpMove) -> String {
    match m {
        OpMove::PMove(mv) => format!("+{}", string_of_move(mv)),
        OpMove::OMove(mv) => format!("-{}", string_of_move(mv)),
    }
}

type Hist = Vec<OpMove>;

fn string_of_hist(x: &Hist) -> String {
    let mut s_hist = String::from("");

    for i in x {
        let j = i.clone();
        s_hist = format!("{}{} ", s_hist, string_of_opmove(j));
    }
    s_hist
}

fn print_hist(x: &Hist) {
    println!("{}", string_of_hist(x));
}

fn print_scores(scores: Vec<(String, (i32, i32, i32))>) {
    for (a, (s, w, l)) in scores {
        println!("{}: {} (Win {}, Lose {})", a, s, w, l);
    }
}

enum State {
    WaitStart,
    MyMove,
    OpMove,
}

fn playing_games(
    state: State,
    reader: &mut BufReader<&TcpStream>,
    writer: &mut BufWriter<&TcpStream>,
    board: &mut Board,
    color: Color,
    hist: &mut Hist,
    oname: &mut String,
    opt_verbose: bool,
    opt_player_name: String,
) {
    match state {
        State::WaitStart => match input_command_multi(reader) {
            Command::Bye(scores) => {
                print_scores(scores);
            }
            Command::Start(color, oname_new, _mytime) => {
                *board = init_board();
                *oname = oname_new;
                if color == black {
                    playing_games(
                        State::MyMove,
                        reader,
                        writer,
                        board,
                        black,
                        &mut Vec::new(),
                        oname,
                        opt_verbose,
                        opt_player_name,
                    );
                } else {
                    playing_games(
                        State::OpMove,
                        reader,
                        writer,
                        board,
                        white,
                        &mut Vec::new(),
                        oname,
                        opt_verbose,
                        opt_player_name,
                    );
                }
            }
            other_commands => {
                println!(
                    "Bye か Start が来ることを予期していますが、実際には{:?}が来ています",
                    other_commands
                );
                panic!("Invalid Command");
            }
        },
        State::MyMove => {
            let pmove = play(board, color);
            do_move(board, &pmove, color);
            output_command(writer, Command::Move(pmove));

            if opt_verbose {
                println!("--------------------------------------------------------------------------------");
                println!(
                    "PMove: {} {}",
                    string_of_move(pmove),
                    string_of_color(color)
                );
                print_board(board);
            }

            match input_command_multi(reader) {
                Command::Ack(_mytime) => {
                    hist.push(OpMove::PMove(pmove));
                    playing_games(
                        State::OpMove,
                        reader,
                        writer,
                        board,
                        color,
                        hist,
                        oname,
                        opt_verbose,
                        opt_player_name,
                    );
                }
                Command::End(wl, n, m, r) => {
                    match wl {
                        Wl::Win => println!("You win! ({} vs. {}) -- {}.", n, m, r),
                        Wl::Lose => println!("You lose! ({} vs. {}) -- {}.", n, m, r),
                        Wl::Tie => println!("Draw ({} vs. {}) -- {}.", n, m, r),
                    }
                    println!(
                        "Your name: {} ({})  Oppnent name: {} ({}).",
                        opt_player_name,
                        string_of_color(color),
                        oname,
                        string_of_color(opposite_color(color))
                    );
                    print_board(board);
                    print_hist(hist);
                    playing_games(
                        State::WaitStart,
                        reader,
                        writer,
                        board,
                        color,
                        hist,
                        oname,
                        opt_verbose,
                        opt_player_name,
                    );
                }
                other_commands => {
                    println!(
                        "Ack か End が来ることを予期していますが、実際には{:?}が来ています",
                        other_commands
                    );
                    panic!("Invalid Command");
                }
            }
        }
        State::OpMove => match input_command_multi(reader) {
            Command::Move(omove) => {
                do_move(board, &omove, opposite_color(color));

                if opt_verbose {
                    println!("--------------------------------------------------------------------------------");
                    println!(
                        "OMove: {} {}",
                        string_of_move(omove),
                        string_of_color(opposite_color(color))
                    );
                    print_board(board);
                }

                hist.push(OpMove::OMove(omove));
                playing_games(
                    State::MyMove,
                    reader,
                    writer,
                    board,
                    color,
                    hist,
                    oname,
                    opt_verbose,
                    opt_player_name,
                );
            }
            Command::End(wl, n, m, r) => {
                match wl {
                    Wl::Win => println!("You win! ({} vs. {}) -- {}.", n, m, r),
                    Wl::Lose => println!("You lose! ({} vs. {}) -- {}.", n, m, r),
                    Wl::Tie => println!("Draw ({} vs. {}) -- {}.", n, m, r),
                }
                println!(
                    "Your name: {} ({})  Oppnent name: {} ({}).",
                    opt_player_name,
                    string_of_color(color),
                    oname,
                    string_of_color(opposite_color(color))
                );
                print_board(board);
                print_hist(hist);
                playing_games(
                    State::WaitStart,
                    reader,
                    writer,
                    board,
                    color,
                    hist,
                    oname,
                    opt_verbose,
                    opt_player_name,
                );
            }
            other_commands => {
                println!(
                    "Move か End が来ることを予期していますが、実際には{:?}が来ています",
                    other_commands
                );
                panic!("Invalid Command");
            }
        },
    }
}

fn main() {
    let matches = App::new("Reversi")
        .arg(
            Arg::with_name("host name")
                .short("H")
                .help("host name (default = local host)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port number")
                .short("p")
                .help("port number (default = 3000)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("player name")
                .help("player name (default = Anon.)")
                .short("n")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbose mode")
                .help("verbose mode")
                .short("v")
                .takes_value(true),
        )
        .get_matches();

    let opt_verbose: bool = match matches.value_of("verbose mode") {
        Some(s) => s.parse().unwrap(),
        None => false,
    };
    let opt_player_name: String = match matches.value_of("player name") {
        Some(s) => s.parse().unwrap(),
        None => String::from("Anon."),
    };
    let opt_port: i32 = match matches.value_of("port number") {
        Some(s) => s.parse().unwrap(),
        None => 3000,
    };
    let opt_host: String = match matches.value_of("host name") {
        Some(s) => s.parse().unwrap(),
        None => String::from("localhost"),
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

                let opt_player_name_clone = opt_player_name.clone();
                let mut reader = BufReader::new(&stream);
                let mut writer = BufWriter::new(&stream);

                output_command(&mut writer, Command::Open(opt_player_name));
                playing_games(
                    State::WaitStart,
                    &mut reader,
                    &mut writer,
                    &mut [[0; 10]; 10],
                    white,
                    &mut Vec::new(),
                    &mut String::new(),
                    opt_verbose,
                    opt_player_name_clone,
                );
            }
        }
    } else {
        eprintln!("Invalid Host:Port Number");
    }
}

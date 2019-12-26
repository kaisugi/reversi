use crate::color::*;
use crate::command::*;
use rand::Rng;

pub type Board = [[Color; 10]; 10];

pub fn init_board() -> Board {
    let mut board = [[0; 10]; 10];
    for i in 0..10 {
        board[i][0] = sentinel;
        board[i][9] = sentinel;
        board[0][i] = sentinel;
        board[9][i] = sentinel;
    }
    board[4][4] = white;
    board[5][5] = white;
    board[4][5] = black;
    board[5][4] = black;

    board
}

fn g(
    board: &Board,
    color: Color,
    (di, dj): (i32, i32),
    (i, j): (i32, i32),
    r: &mut Vec<(i32, i32)>,
) {
    let ocolor = opposite_color(color);

    if board[i as usize][j as usize] == ocolor {
        r.push((i, j));
        g(board, color, (di, dj), (i + di, j + dj), r);
    } else if board[i as usize][j as usize] == color {
    } else {
        r.clear();
    }
}

fn f(
    board: &Board,
    color: Color,
    (di, dj): (i32, i32),
    (i, j): (i32, i32),
    r: &mut Vec<(i32, i32)>,
) {
    let ocolor = opposite_color(color);

    if board[i as usize][j as usize] == ocolor {
        r.push((i, j));
        g(board, color, (di, dj), (i + di, j + dj), r)
    } else {
        r.clear();
    }
}

pub fn flippable_indices_line(
    board: &Board,
    color: Color,
    (di, dj): (i32, i32),
    (i, j): (i32, i32),
) -> Vec<(i32, i32)> {
    let mut tmp = Vec::new();
    f(board, color, (di, dj), (i, j), &mut tmp);
    (*tmp).to_vec()
}

pub fn flippable_indices(board: &Board, color: Color, (i, j): (i32, i32)) -> Vec<(i32, i32)> {
    let dirs = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let mut bs = Vec::new();
    for (di, dj) in dirs {
        bs.append(&mut flippable_indices_line(
            board,
            color,
            (di, dj),
            (i + di, j + dj),
        ));
    }
    bs
}

pub fn is_effective(board: &Board, color: Color, (i, j): (i32, i32)) -> bool {
    if flippable_indices(board, color, (i, j)).is_empty() {
        false
    } else {
        true
    }
}

pub fn is_valid_move(board: &Board, color: Color, (i, j): (i32, i32)) -> bool {
    board[i as usize][j as usize] == none && is_effective(board, color, (i, j))
}

pub fn do_move(board: &mut Board, com: &Move, color: Color) {
    match com {
        Move::Mv(i, j) => {
            let ms = flippable_indices(board, color, (*i, *j));
            for (ii, jj) in &ms {
                board[*ii as usize][*jj as usize] = color;
            }
            board[*i as usize][*j as usize] = color;
        }
        _ => {}
    }
}

pub fn mix(xs: Vec<i32>, ys: Vec<i32>) -> Vec<(i32, i32)> {
    let mut ans_v = Vec::new();
    for i in &xs {
        for j in &ys {
            ans_v.push((*i, *j));
        }
    }
    ans_v
}

pub fn valid_moves(board: &Board, color: Color) -> Vec<(i32, i32)> {
    let ls1 = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let ls2 = vec![1, 2, 3, 4, 5, 6, 7, 8];

    let mut ans_v = Vec::new();
    for tup in &mix(ls1, ls2) {
        if is_valid_move(board, color, *tup) {
            ans_v.push(*tup);
        }
    }
    ans_v
}

pub fn play(board: &Board, color: Color) -> Move {
    let ms = valid_moves(board, color);
    if ms == vec![] {
        Move::Pass
    } else {
        let mut rng = rand::thread_rng();
        let k = rng.gen_range(0, ms.len());
        let (i, j) = ms[k];
        Move::Mv(i, j)
    }
}

pub fn print_board(board: &Board) {
    println!(" |A B C D E F G H ");
    println!("-+----------------");

    for j in 1..9 {
        print!("{}|", j);
        for i in 1..9 {
            print_color(board[i][j]);
            print!(" ");
        }
        print!("\n");
    }
    println!("  (X: Black,  O: White)");
}

#[test]
fn check() {
    let mut board = init_board();
    assert_eq!(is_effective(&board, black, (3, 4)), true);
    assert_eq!(is_effective(&board, black, (3, 5)), false);
    assert_eq!(is_effective(&board, black, (2, 5)), false);
    assert_eq!(is_effective(&board, black, (6, 6)), false);
    assert_eq!(is_effective(&board, white, (6, 6)), false);
    assert_eq!(is_effective(&board, white, (4, 6)), true);

    for i in 1..9 {
        for j in 1..9 {
            board[i][j] = white;
        }
    }
    board[1][1] = black;
    board[6][3] = black;
    board[8][1] = black;
    assert_eq!(
        flippable_indices(&board, black, (1, 8)),
        vec![
            (1, 7),
            (1, 6),
            (1, 5),
            (1, 4),
            (1, 3),
            (1, 2),
            (2, 7),
            (3, 6),
            (4, 5),
            (5, 4)
        ]
    );
    assert_eq!(
        flippable_indices(&board, black, (6, 1)),
        vec![(5, 1), (4, 1), (3, 1), (2, 1), (7, 1), (6, 2)]
    );

    board = init_board();
    print_board(&board);
    assert_eq!(
        valid_moves(&board, black),
        vec![(3, 4), (4, 3), (5, 6), (6, 5)]
    );
    for i in 4..9 {
        for j in 3..9 {
            board[i][j] = white;
        }
    }
    board[8][8] = black;
    board[5][4] = black;
    print_board(&board);
    assert_eq!(
        valid_moves(&board, black),
        vec![
            (3, 2),
            (3, 3),
            (3, 4),
            (3, 6),
            (3, 8),
            (5, 2),
            (7, 2),
            (8, 2)
        ]
    );
}

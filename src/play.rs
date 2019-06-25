use crate::color::*;

pub type Board = Vec<Vec<Color>>;

pub fn init_board () -> Board {
  let mut board = vec![vec![0; 10]; 10];
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

fn g (board: &Board, color: Color, (di, dj): (i32, i32), (i, j): (i32, i32), r: &mut Vec<(i32, i32)>) {
  let ocolor = opposite_color(color);

  if board[i as usize][j as usize] == ocolor {
    r.push((i, j));
    g(board, color, (di, dj), (i+di, j+dj), r);
  } else if board[i as usize][j as usize] == color {
  } else {
    r.clear();
  }
}

fn f (board: &Board, color: Color, (di, dj): (i32, i32), (i, j): (i32, i32), r: &mut Vec<(i32, i32)>) {
  let ocolor = opposite_color(color);

  if board[i as usize][j as usize] == ocolor {
    r.push((i, j));
    g(board, color, (di, dj), (i+di, j+dj), r)
  } else {
    r.clear();
  }
}

pub fn flippable_indices_line (board: &Board, color: Color, (di, dj): (i32, i32), (i, j): (i32, i32)) -> Vec<(i32, i32)> {
  let mut tmp = Vec::new();
  f(board, color, (di, dj), (i, j), &mut tmp);
  (*tmp).to_vec()
}

pub fn flippable_indices (board: &Board, color: Color, (i, j): (i32, i32)) -> Vec<(i32, i32)> {
  let dirs = vec![(-1,-1), (0, -1), (1,-1), (-1,0), (1,0), (-1,1), (0,1), (1,1)];

  let mut bs = Vec::new();
  for (di, dj) in dirs {
    bs.append(&mut flippable_indices_line(board, color, (di, dj), (i+di, j+dj)));
  }
  bs
}

pub fn is_effective (board: &Board, color: Color, (i, j): (i32, i32)) -> bool {
  if flippable_indices(board, color, (i, j)).is_empty() {
    false
  } else {
    true
  }
}

pub fn is_valid_move (board: &Board, color: Color, (i, j): (i32, i32)) -> bool {
  board[i as usize][j as usize] == none && is_effective(board, color, (i, j))
}


#[test]
fn check() {
  let board = init_board();
  assert_eq!(is_effective(&board, black, (3, 4)), true);
  assert_eq!(is_effective(&board, black, (3, 5)), false);
  assert_eq!(is_effective(&board, black, (2, 5)), false);
  assert_eq!(is_effective(&board, black, (6, 6)), false);
  assert_eq!(is_effective(&board, white, (6, 6)), false);
  assert_eq!(is_effective(&board, white, (4, 6)), true);
}
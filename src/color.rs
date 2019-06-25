type Color = i32;

pub fn opposite_color(c: Color) -> Color {
  (2 - c) + 1
}

pub fn string_of_color(c: Color) -> String {
  if c == 1 {
    "White".to_string()
  } else {
    "Black".to_string()
  }
}

pub fn print_color(c: Color) {
  if c == 1 {
    print!("O")
  } else if c == 2 {
    print!("X")
  } else if c == 0 {
    print!(" ")
  } 
}
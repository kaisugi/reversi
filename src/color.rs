pub type Color = i32;

#[allow(non_upper_case_globals)]
pub static none: Color = 0;
#[allow(non_upper_case_globals)]
pub static white: Color = 1;
#[allow(non_upper_case_globals)]
pub static black: Color = 2;
#[allow(non_upper_case_globals)]
pub static sentinel: Color = 3;

pub fn opposite_color(c: Color) -> Color {
    (2 - c) + 1
}

pub fn string_of_color(c: Color) -> String {
    if c == white {
        "White".to_string()
    } else {
        "Black".to_string()
    }
}

pub fn print_color(c: Color) {
    if c == white {
        print!("O")
    } else if c == black {
        print!("X")
    } else if c == none {
        print!(" ")
    }
}

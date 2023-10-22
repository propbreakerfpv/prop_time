use chrono::{Local, Datelike};



// \u{1b}[38;5;{}m{}\u{1b}[39m

fn main() {
    let a = Local::now();
    let mon_color = match a.month() {
        1 => format!("\u{1b}[38;2;148;196;247m{}\u{1b}[39m", a.format("%a%b%d")),
        2 => format!("\u{1b}[38;2;4;81;247m{}\u{1b}[39m", a.format("%a%b%d")),
        3 => format!("\u{1b}[38;2;0;154;118m{}\u{1b}[39m", a.format("%a%b%d")), // 009A76
        4 => format!("\u{1b}[38;2;233;242;150m{}\u{1b}[39m", a.format("%a%b%d")),
        5 => format!("\u{1b}[38;2;186;97;216m{}\u{1b}[39m", a.format("%a%b%d")),
        6 => format!("\u{1b}[38;2;153;245;98m{}\u{1b}[39m", a.format("%a%b%d")),
        7 => format!("\u{1b}[38;2;242;217;26m{}\u{1b}[39m", a.format("%a%b%d")),
        8 => format!("\u{1b}[38;2;242;120;26m{}\u{1b}[39m", a.format("%a%b%d")),
        9 => format!("\u{1b}[38;2;188;77;16m{}\u{1b}[39m", a.format("%a%b%d")),
        10 => format!("\u{1b}[38;2;175;116;52m{}\u{1b}[39m", a.format("%a%b%d")),
        11 => format!("\u{1b}[38;2;127;116;110m{}\u{1b}[39m", a.format("%a%b%d")),
        12 => format!("\u{1b}[38;2;9;86;28m{}\u{1b}[39m", a.format("%a%b%d")),
        m => panic!("it is month {}!!!", m),
    };
    println!("{} {}", mon_color, a.format("%H:%M"));
}

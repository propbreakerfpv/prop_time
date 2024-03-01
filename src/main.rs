use std::env::args;

use chrono::{Local, Datelike, Utc, DateTime, Duration};
use chrono_tz::{US::Eastern, Tz};


struct Config {
    aoc: bool,
}

impl Config {
    fn new() -> Config {
        Config {
            aoc: false,
        }
    }
}

// \u{1b}[38;5;{}m{}\u{1b}[39m

fn main() {
    let config = create_config();

    let a = Local::now();
    let month_colored = match a.month() {
        1 => format!("\u{1b}[38;2;148;196;247m{}\u{1b}[39m", a.format("%a%b%d")),
        2 => format!("\u{1b}[38;2;4;81;247m{}\u{1b}[39m", a.format("%a%b%d")),
        3 => {
            let date: DateTime<Tz> = Utc::now().with_timezone(&Eastern);
            let day = date.day();
            match day {
                14 => {
                    format!("\u{1b}[38;2;0;154;118mPi Day{}\u{1b}[39m", a.format("%a%b%d"))
                }
                _ => format!("\u{1b}[38;2;0;154;118m{}\u{1b}[39m", a.format("%a%b%d")), // 009A76
            }
        }
        4 => format!("\u{1b}[38;2;233;242;150m{}\u{1b}[39m", a.format("%a%b%d")),
        5 => {
            let date: DateTime<Tz> = Utc::now().with_timezone(&Eastern);
            let day = date.day();
            match day {
                4 => format!("\u{1b}[38;2;186;97;216mMay the Forth Be With You\u{1b}[39m"),
                _ => format!("\u{1b}[38;2;186;97;216m{}\u{1b}[39m", a.format("%a%b%d")),
            }
        }
        6 => format!("\u{1b}[38;2;153;245;98m{}\u{1b}[39m", a.format("%a%b%d")),
        7 => format!("\u{1b}[38;2;242;217;26m{}\u{1b}[39m", a.format("%a%b%d")),
        8 => format!("\u{1b}[38;2;242;120;26m{}\u{1b}[39m", a.format("%a%b%d")),
        9 => format!("\u{1b}[38;2;188;77;16m{}\u{1b}[39m", a.format("%a%b%d")),
        10 => format!("\u{1b}[38;2;175;116;52m{}\u{1b}[39m", a.format("%a%b%d")),
        11 => format!("\u{1b}[38;2;113;137;158m{}\u{1b}[39m", a.format("%a%b%d")),
        12 => {
            let date: DateTime<Tz> = Utc::now().with_timezone(&Eastern);
            let day = date.day();
            // i do not give a FUCK
            #[allow(deprecated)]
            let midnight = (date + Duration::days(1)).date().and_hms(0, 0, 0);
            let duration = midnight.signed_duration_since(date).to_std().unwrap();
            
            let dur_sec = duration.as_secs() % 60;
            let dur_min = (duration.as_secs()) / 60 % 60;
            let dur_hour = ((duration.as_secs()) / 60) / 60 % 60;
            if day <= 25 && config.aoc {
                format!("\u{1b}[38;2;2;180;4mAoC day{} in {}:{}:{}\u{1b}[38;2;9;86;28m {}\u{1b}[39m", day + 1, dur_hour, dur_min, dur_sec, a.format("%a%b%d"))
            } else {
                format!("\u{1b}[38;2;9;86;28m{}\u{1b}[39m", a.format("%a%b%d"))
            }
        }
        m => format!("it is month {}!?!?!?", m),
    };
    println!("{} {}", month_colored, a.format("%H:%M"));
}

fn create_config() -> Config {
    let mut config = Config::new();
    let args = args();
    for arg in args {
        match &arg[..] {
            "aoc" => config.aoc = true,
            _ => {}
        };
    }
    config
}

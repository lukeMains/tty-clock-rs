use std::env;
use std::io::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use chrono::{DateTime, Local};
use signal_hook::{consts::TERM_SIGNALS, flag};

const DATEWINH: i32 = 3;

enum ColorPairs {
    Normal = 0,
    Inverted = 1,
}

#[allow(dead_code)]
enum AnsiColors {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

#[allow(dead_code)]
enum TimeFormat {
    TwelveHour,
    TwentyFourHour,
}

const NUMBER_MATRIX: [[i16; 15]; 10] = [
    [1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1], /* 0 */
    [0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1], /* 1 */
    [1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1], /* 2 */
    [1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1], /* 3 */
    [1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1], /* 4 */
    [1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1], /* 5 */
    [1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1], /* 6 */
    [1, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1], /* 7 */
    [1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1], /* 8 */
    [1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1], /* 9 */
];

fn init() -> (ncurses::WINDOW, ncurses::WINDOW) {
    // Set locale
    ncurses::setlocale(ncurses::LcCategory::all, "");

    // Initialize Screen
    ncurses::initscr();
    ncurses::cbreak();
    ncurses::noecho();
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    ncurses::clear();

    // Setup color pairs
    ncurses::start_color();
    let default = if ncurses::OK == ncurses::use_default_colors() {
        -1
    } else {
        ncurses::COLOR_BLACK
    };

    // Negative one (-1) means default foreground/background
    ncurses::init_pair(ColorPairs::Normal as i16, ncurses::COLOR_GREEN, default);
    ncurses::init_pair(ColorPairs::Inverted as i16, default, ncurses::COLOR_GREEN);

    // Make time window
    let lines = 7; // Default Height
    let cols = 54; // Default Width
    let x = 0;
    let y = 0;
    let timewin = ncurses::newwin(lines, cols, y, x); // Start in top left corner.

    let (_, _, _, datestr) = get_time(TimeFormat::TwelveHour); // !FIXME: variable time format

    // Make date window
    let datestr_len: i32 = datestr.len().try_into().unwrap();
    let datewin = ncurses::newwin(
        DATEWINH,
        datestr_len + 2,
        y + lines - 1,
        x + (cols / 2) - (datestr_len / 2) - 1,
    );

    (timewin, datewin)
}

fn get_time(format: TimeFormat) -> ((usize, usize), (usize, usize), (usize, usize), String) {
    let dt: DateTime<Local> = Local::now();

    let hour = match format {
        TimeFormat::TwelveHour => dt.format("%I"),
        TimeFormat::TwentyFourHour => dt.format("%H"),
    }
    .to_string();

    let min = dt.format("%M").to_string();

    let sec = dt.format("%S").to_string();

    let datestr = match format {
        TimeFormat::TwelveHour => dt.format("%F [%p]"),
        TimeFormat::TwentyFourHour => dt.format("%F"),
    }
    .to_string();

    // Parse Numbers
    // TODO: Should I parse once and do math or split the string
    // and parse twice?
    let h1: usize = hour.as_str()[0..1].parse().unwrap();
    let h2: usize = hour.as_str()[1..2].parse().unwrap();

    let m1: usize = min.as_str()[0..1].parse().unwrap();
    let m2: usize = min.as_str()[1..2].parse().unwrap();

    let s1: usize = sec.as_str()[0..1].parse().unwrap();
    let s2: usize = sec.as_str()[1..2].parse().unwrap();

    ((h1, h2), (m1, m2), (s1, s2), datestr)
}

fn draw_number(window: ncurses::WINDOW, n: usize, x: i32, y: i32) {
    // TODO: Better variable names
    let mut sy = y;
    let mut sx = x;

    for i in 0..15 {
        if sy == y + 6 {
            sy = y;
            sx += 1;
        }

        ncurses::wbkgdset(window, ncurses::COLOR_PAIR(NUMBER_MATRIX[n][i]));
        ncurses::wmove(window, sx, sy);
        // ncurses::waddch(w, '█' as u32);
        ncurses::waddstr(window, "  ");

        sy += 2;
    }
    ncurses::wrefresh(window);
}

fn draw_clock(
    window: ncurses::WINDOW,
    hour: (usize, usize),
    min: (usize, usize),
    sec: (usize, usize),
) {
    // Hours
    draw_number(window, hour.0, 1, 1);
    draw_number(window, hour.1, 1, 8);

    // Dots
    ncurses::wmove(window, 2, 16);
    ncurses::waddstr(window, "  ");
    ncurses::wmove(window, 4, 16);
    ncurses::waddstr(window, "  ");

    // Minutes
    draw_number(window, min.0, 1, 20);
    draw_number(window, min.1, 1, 27);

    // TODO: Make seconds optional...
    // Dots
    ncurses::wmove(window, 2, 35);
    ncurses::waddstr(window, "  ");
    ncurses::wmove(window, 4, 35);
    ncurses::waddstr(window, "  ");

    // Seconds
    draw_number(window, sec.0, 1, 39);
    draw_number(window, sec.1, 1, 46);

    ncurses::wrefresh(window);
}

fn draw_date(window: ncurses::WINDOW, datestr: &str) {
    ncurses::wbkgdset(window, ncurses::COLOR_PAIR(ColorPairs::Normal as i16));
    ncurses::mvwprintw(window, DATEWINH / 2, 1, datestr);
    ncurses::wrefresh(window);
}

fn _clock_move(_x: i32, _y: i32, _w: i32, _h: i32) {}

fn _set_second() {}

fn _set_center(_b: bool) {}

fn _set_box(_b: bool) {}

fn _key_event() {}

fn usage() {
    println!("usage : tty-clock-rs [-iuvsScbtrahDBxn] [-C [0-7]] [-f format] [-d delay] [-a nsdelay] [-T tty] ");
    println!("    -s            Show seconds                                   ");
    println!("    -S            Screensaver mode                               ");
    println!("    -x            Show box                                       ");
    println!("    -c            Set the clock at the center of the terminal    ");
    println!("    -C [0-7]      Set the clock color                            ");
    println!("    -b            Use bold colors                                ");
    println!("    -t            Set the hour in 12h format                     ");
    println!("    -u            Use UTC time                                   ");
    println!("    -T tty        Display the clock on the specified terminal    ");
    println!("    -r            Do rebound the clock                           ");
    println!("    -f format     Set the date format                            ");
    println!("    -n            Don't quit on keypress                         ");
    println!("    -v            Show tty-clock version                         ");
    println!("    -i            Show some info about tty-clock                 ");
    println!("    -h            Show this page                                 ");
    println!("    -D            Hide date                                      ");
    println!("    -B            Enable blinking colon                          ");
    println!("    -d delay      Set the delay between two redraws of the clock. Default 1s. ");
    println!("    -a nsdelay    Additional delay between two redraws in nanoseconds. Default 0ns.");
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => (),
        _ => {
            // Parse arguments starting after the program name.
            for arg in args[1..].iter() {
                match arg.as_str() {
                    "-h" | "--help" => {
                        usage();
                        return Ok(());
                    }
                    _ => (), // TODO: Add arguments form usage()
                }
            }
        }
    }

    let (timewin, datewin) = init();

    // Main Event Loop
    let terminate = Arc::new(AtomicBool::new(false));

    // Setup Signal Handlers
    // TODO: Add SIGSTP (Ctrl-Z functionality)
    TERM_SIGNALS.iter().for_each(|&signal| {
        flag::register(signal, Arc::clone(&terminate)).unwrap();
    });

    while !terminate.load(Ordering::Relaxed) {
        let (hour, min, sec, datestr) = get_time(TimeFormat::TwelveHour);
        draw_date(datewin, &datestr);
        draw_clock(timewin, hour, min, sec);
        ncurses::refresh(); // Update the screen.
    }

    ncurses::endwin(); // Terminate ncurses.

    Ok(())
}

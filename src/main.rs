use std::env;
use std::io::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

//use ncurses;
use signal_hook::{consts::TERM_SIGNALS, flag};

enum ColorPairs {
    NoColor = 0,
    Color,
    Inverted,
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

fn init() {
    /* If your locale env is unicode, you should use `setlocale`. */
    let locale_conf = ncurses::LcCategory::all;
    ncurses::setlocale(locale_conf, "en_US.UTF-8");

    // Initialize Screen
    ncurses::initscr();
    ncurses::cbreak();
    ncurses::noecho();
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    ncurses::clear();

    // Setup color pairs
    ncurses::start_color();
    ncurses::init_pair(
        ColorPairs::NoColor as i16,
        ncurses::COLOR_BLACK,
        ncurses::COLOR_BLACK,
    );
    ncurses::init_pair(
        ColorPairs::Color as i16,
        ncurses::COLOR_BLACK,
        ncurses::COLOR_GREEN,
    );
    ncurses::init_pair(
        ColorPairs::Inverted as i16,
        ncurses::COLOR_GREEN,
        ncurses::COLOR_BLACK,
    );
}

fn _update_hour() {}

fn draw_number(w: ncurses::WINDOW, n: usize, x: i32, y: i32) {
    // TODO: Better variable names
    let mut sy = y;
    let mut sx = x;

    for i in 0..30 {
        if sy == y + 6 {
            sy = y;
            sx += 1;
        }

        ncurses::wbkgdset(w, ncurses::COLOR_PAIR(NUMBER_MATRIX[n][i / 2]));
        ncurses::wmove(w, sx, sy);
        ncurses::waddch(w, ' ' as u32);

        sy += 1;
    }
    ncurses::wrefresh(w);
}

fn _draw_clock() {}

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

    init();

    // Main Event Loop
    let terminate = Arc::new(AtomicBool::new(false));

    // Setup Signal Handlers
    // TODO: Add SIGSTP (Ctrl-Z functionality)
    TERM_SIGNALS.iter().for_each(|&signal| {
        flag::register(signal, Arc::clone(&terminate)).unwrap();
    });

    // Make a window
    let lines = 7; // Default Height
    let cols = 35; // Default Width
    let x = 0;
    let y = 0;
    let framewin = ncurses::newwin(lines, cols, y, x); // Start in top left corner.
    while !terminate.load(Ordering::Relaxed) {
        draw_number(framewin, 0, 1, 1);
        ncurses::refresh(); // Update the screen.
    }

    ncurses::endwin(); // Terminate ncurses.

    Ok(())
}

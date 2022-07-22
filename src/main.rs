use std::env;

fn _init() {}

fn _signal_handler(_signal: i8) {}

fn _update_hour() {}

fn _draw_number(_n: i8, _x: i8, _y: i8) {}

fn _draw_clock() {}

fn _clock_move(_x: i8, _y: i8, _w: i8, _h: i8) {}

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

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => usage(),
        _ => {
            // Parse arguments starting after
            for arg in args[1..].into_iter() {
                match arg.as_str() {
                    "-h" | "--help" => {
                        usage();
                        break;
                    }
                    _ => (),
                }
            }
        }
    }
}

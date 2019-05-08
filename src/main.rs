#![feature(duration_float)]
use std::{error::Error, thread, time::Duration};
use termion::{color, style};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::async_stdin;
use std::io::{Write, Read,stdout, stdin};
use structopt::StructOpt;


fn main() -> Result<(), Box<dyn Error>>{


    let opt = tread::Opt::from_args();

    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode()?;
    let mut stdin = async_stdin().keys();

    write!(stdout, "{}{}{}",
           // Clear the screen.
           termion::clear::All,
           // Goto (1,1).
           termion::cursor::Goto(1, 1),
           // Hide the cursor.
           termion::cursor::Hide)?;

    // A minute
    let mut MINUTE_SECS: f64 = 60.0;

    // Speed of WPM interpolation
    let mut WPM_SPEED: f64 = 10.0;
    // Current MAXimum WPM
    let mut MAX_WPM: f64 = 300.0;
    /// Starting WPM
    const INITIAL_WPM: f64 = 200.0;
    /// How much each keypress changes the WPM by
    const WPM_INCREMENT: f64 = 50.0;




    // Create SpeedReader
    let mut sr = tread::SpeedReader::new(opt.files[0].clone())?;

    // Setup WPM
    let mut wpm: f64 = INITIAL_WPM;

    // State
    let mut hide = false;
    let mut pause = false;

    loop {
        let (width, height) = termion::terminal_size()?;


        // Clear the screen
        write!(stdout, "{}", termion::clear::All)?;

        match stdin.next() {
            Some(key) => {
                match key? {
                    // Change WPM
                    Key::Up => {
                        if MAX_WPM < 1000.0 {
                            MAX_WPM += WPM_INCREMENT
                        }
                    },
                    Key::Down => { 
                        if MAX_WPM > 101.0 {
                            MAX_WPM -= WPM_INCREMENT
                        }
                    },

                    // Quit
                    Key::Esc => break,
                    Key::Char('q') => break,
                    // hide the status bar
                    Key::Char('h') => hide = !hide,
                    // Restart the text
                    Key::Char('r') => sr.index = 0,
                    // Pause function
                    Key::Char(' ') => {
                        wpm = INITIAL_WPM;
                        pause = !pause;

                        if hide == true {
                            hide = false;
                        }
                    },

                    // Rewind
                    Key::Left => {
                        pause = true;
                        sr.prev();
                    },
                    // Fast-forward
                    Key::Right => {
                        pause = true;
                        sr.next();
                    },
                    _ => {},
                }
            }
            _ => {},
        }

        if !hide {
            // Terminal Status bar
            let status_bar = format!("WPM: {} WORD: {}/{} TIME LEFT: {}m, PAUSED: {}", wpm, sr.index, sr.words.len(), sr.words.len() / wpm as usize, pause);
            let status_width = (width / 2) as isize - status_bar.len() as isize / 2;

            if status_width <= 0 {
                // hide = true;
            }
            else {
                write!(stdout, "{}{}{}{}", color::Fg(color::Rgb(0, 255,0)), termion::cursor::Goto(status_width as u16, 0), status_bar, color::Fg(color::Reset))?;
            }
        }

        let word = sr.get_word();
        write!(stdout, "{}{}", termion::cursor::Goto(width / 2 - (word.len() / 2) as u16, height / 2), word)?;

        if !pause {
            if wpm < MAX_WPM {
                wpm += WPM_SPEED;
            }
            else {
                wpm = MAX_WPM;
            }

                sr.next();

            stdout.flush()?;
            thread::sleep(Duration::from_secs_f64(MINUTE_SECS / wpm ));
        }
        else {
            stdout.flush()?;
        }
    }

    write!(stdout, "{}{}{}",
           // Clear the screen.
           termion::clear::All,
           // Goto (1,1).
           termion::cursor::Goto(1, 1),
           // Hide the cursor.
           termion::cursor::Show)?;

    Ok(())
}

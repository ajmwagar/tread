#![feature(duration_float)]
use std::{error::Error, thread, time::Duration};
use termion::{color};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::async_stdin;
use std::io::{Write, stdout};
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
    const MINUTE_SECS: f64 = 60.0;

    // Speed of WPM interpolation
    const WPM_SPEED: f64 = 10.0;
    // Current minimum
    const MIN_WPM: f64 = 200.0;

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


    let mut old_word = String::new();
    loop {
        let (width, height) = termion::terminal_size()?;

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
                        if pause != true {
                            pause = true;
                        }
                        sr.prev();
                    },
                    // Fast-forward
                    Key::Right => {
                        if pause != true {
                            pause = true;
                        }
                        sr.next();
                    },
                    _ => {
                    },
                }
            }
            _ => {},
        }


        // Clear the screen
        // write!(stdout, "{}", termion::clear::All)?;

        if !hide {
            // Terminal Status bar
            let status_bar = format!("WPM: {} MAX_WPM: {} WORD: {}/{} TIME LEFT: {}m, PAUSED: {}", wpm, MAX_WPM, sr.index, sr.words.len(), sr.words.len() / wpm as usize, pause);
            let status_width = (width / 2) as isize - status_bar.len() as isize / 2;


            if status_width <= 0 {
                // hide = true;
            }
            else {
                
                write!(stdout, "{}{}{}{}{}", color::Fg(color::Rgb(0, 255,0)), termion::cursor::Goto(status_width as u16, 0), termion::clear::CurrentLine, status_bar, color::Fg(color::Reset))?;
                stdout.flush()?;
            }
        }


        let word = sr.get_word();

        if old_word != word {
            write!(stdout, "{}{}{}", termion::cursor::Goto(width / 2 - (word.len() / 2) as u16, height / 2), termion::clear::CurrentLine, word)?;
            stdout.flush()?;
        }

        old_word = word;

        if !pause {
            if wpm < MAX_WPM {
                wpm += WPM_SPEED;
            }
            else {
                wpm = MAX_WPM;
            }

            if wpm <= MIN_WPM {
                wpm = MIN_WPM;
            }

            sr.next();

        }

        thread::sleep(Duration::from_secs_f64(MINUTE_SECS / wpm ));

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

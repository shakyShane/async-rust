//! # Talk Timer
//! This is a simple command line tool that displays a timer. Built as a toy to
//! explore typestates in Rust, and used as a prompt for speakers during talks.

use std::io::{self, Write};
use std::time::{Duration, Instant};
use std::{thread};
use crossbeam::channel::{RecvError, Sender};

struct Init;
struct Countdown;
struct LastStretch;
struct Done;

struct Timer<S> {
    duration: Duration,
    start: Instant,
    sender: Sender<Progress>,
    _state: S,
}

impl Timer<Init> {
    /// Transition to countdown.
    ///
    /// This method consumes the sender in its current state,
    /// returns it in a new state.
    fn start(self) -> Timer<Countdown> {
        let sec_remaining = (self.duration - self.start.elapsed()).as_secs();

        self.sender.send(Progress::Time(format!("\r{:02}:{:02} left", sec_remaining / 60, sec_remaining % 60,))).expect("121");
        io::stdout().flush().unwrap();
        Timer {
            duration: self.duration,
            start: self.start,
            sender: self.sender,
            _state: Countdown,
        }
    }
}

impl Timer<Countdown> {
    /// Run down the timer until time is out.
    fn run(self) -> Timer<LastStretch> {
        loop {
            thread::sleep(Duration::from_millis(100));
            let remaining = self.duration - self.start.elapsed();
            match remaining.as_secs() {
                s if s < 11 => {
                    break;
                }
                sec_remaining => {
                    if (sec_remaining % 10) == 0 {
                        self.sender.send(Progress::Time(format!("\r{:02}:{:02} left", sec_remaining / 60, sec_remaining % 60,))).expect("121");
                        io::stdout().flush().unwrap();
                    }
                }
            }
        }

        Timer {
            duration: self.duration,
            start: self.start,
            sender: self.sender,
            _state: LastStretch,
        }
    }
}

impl Timer<LastStretch> {
    /// Run down the timer until time is out.
    fn run(self) -> Timer<Done> {
        loop {
            thread::sleep(Duration::from_millis(100));
            let remaining = self.duration - self.start.elapsed();
            match remaining.as_secs() {
                0 => {
                    self.sender.send(Progress::Time(format!("\r00:00 left"))).expect("senig");
                    io::stdout().flush().unwrap();
                    break;
                }
                sec_remaining => {
                    let s = format!("\r{:02}:{:02} left", sec_remaining / 60, sec_remaining % 60,);
                    self.sender.send(Progress::Time(s)).expect("ending");
                    io::stdout().flush().unwrap();
                }
            }
        }

        Timer {
            duration: self.duration,
            start: self.start,
            sender: self.sender,
            _state: Done,
        }
    }
}

impl Timer<Done> {
    /// Flash warning until program is killed
    fn wait(&self) {
        // println!("all done!")
    }
}

enum Progress {
    Start,
    Time(String),
    End
}

fn main()  {

    // Create a channel of unbounded capacity.
    let (s, r) = crossbeam::channel::unbounded::<Progress>();

// Send a message into the channel.
//     s.send("Hello, world!").unwrap();

// Receive the message from the channel.
//     assert_eq!(r.recv(), Ok("Hello, world!"));

    // Arguments can be parsed in any order.

    thread::spawn(move || {
        let duration = Duration::from_secs(5);
        let start = Instant::now();
        let timer = Timer {
            duration,
            start,
            sender: s.clone(),
            _state: Init,
        };
        s.send(Progress::Start).expect("sending");
        let countdown = timer.start();
        let laststretch = countdown.run();
        let done = laststretch.run();
        s.send(Progress::End).expect("sending");
        done.wait();
    });
    let handle_2 = thread::spawn(move || {
        loop {
            match r.recv() {
                Ok(Progress::Start) => println!("GO"),
                Ok(Progress::Time(time)) => println!("TIME: {}", time),
                Ok(Progress::End) => {
                    println!("END");
                    break;
                },
                Err(_e) => {
                    println!("err")
                },
            };
        }
    });
    handle_2.join().expect("complete");
}


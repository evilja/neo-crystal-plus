
extern crate pancurses;
extern crate glob;
use std::thread;
use std::time::{Duration, Instant};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
mod modules;
use crate::modules::curses::init_locale;
use crate::modules::{audio::play_audio, crystal_manager::crystal_manager};


fn main() { // establish communications and threads, then give the job to crystal_manager fn
    unsafe { init_locale(); }

    let (tx, rx): (Sender<(&'static str, String)>, Receiver<(&'static str, String)>)                = mpsc::channel();
    let (tx_proc, rx_proc): (Sender<Instant>, Receiver<Instant>)                                    = mpsc::channel();
    let (comm_tx, comm_rx): (Sender<(&'static str, Duration)>, Receiver<(&'static str, Duration)>)  = mpsc::channel();
    let (sigkill, issigkill): (Sender<bool>, Receiver<bool>)                                        = mpsc::channel();
    let mut found_val                                                                               = (false, Instant::now());
    let ret_value: Result<Instant, TryRecvError>                                                    = Err(TryRecvError::Empty);
    thread::spawn(move || {
        match play_audio(rx, tx_proc) {
            Ok(_) => {
                ()
            },
            Err(e) => {
                eprintln!("Error in audio playback: {}", e);
            }
        }
    });

    tx.send(("volume_df", String::new())).unwrap();
    let thrloop: thread::JoinHandle<()> = thread::spawn(move || loop {
        match issigkill.try_recv() {
            Ok(_) => {
                println!("Killing loop thread");
                break;
            },
            Err(_) => (),
        }
        match rx_proc.try_recv() {
            Ok(val) => {
                found_val = (true, val);
                if val <= Instant::now() {
                    found_val = (false, Instant::now());
                }
            },
            Err(_) => (),
        }
        if found_val.0 == false {
            match ret_value {
                Ok(val) => {
                    found_val = (true, val);
                },
                Err(_) => (),
            }
        } else {
            if Instant::now() >= found_val.1 {
                comm_tx.send(("turn", Instant::now() - Instant::now())).unwrap();
                found_val = (false, Instant::now());
            }
            comm_tx.send(("duration", found_val.1 - Instant::now())).unwrap();
        }
        
        thread::sleep(Duration::from_millis(100));
    });

    if crystal_manager(tx, comm_rx) {
        sigkill.send(true).unwrap();
    }
    thrloop.join().unwrap();

}


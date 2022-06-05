use crate::error;
use crate::password;
use crossbeam_channel::{bounded, select};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::ffi::OsString;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;

const HIBP_TOTAL: u64 = 16u64.pow(5);

#[derive(Copy, Clone)]
enum Message {
    Progress,
    Done,
    Error(u64),
}

pub fn downloader(output: OsString) {
    let (sender, receiver) = bounded::<Message>(128);
    let sender = Arc::new(sender);
    let progress_bar = ProgressBar::new(HIBP_TOTAL);
    let mut progress = 0;

    progress_bar.set_style(
        ProgressStyle::template(
            ProgressStyle::default_bar(),
            "[{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>7}/{lens:7} ({eta})",
        )
        .progress_chars("#>-"),
    );

    let thread = thread::spawn(move || {
        let file = std::fs::File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(output);

        if let Ok(file) = file {
            let file = Arc::new(Mutex::new(std::io::BufWriter::new(file)));
            let agent = ureq::agent();

            _ = (0..=HIBP_TOTAL).into_par_iter().try_for_each(|n| {
                let sender = Arc::clone(&sender);
                let file = Arc::clone(&file);
                let result = password::download_range(&agent, n);
                match result {
                    Ok(range) => {
                        let mut file = file.lock().unwrap();
                        let data_to_write = range.as_bytes();
                        let write_output = file.write_all(data_to_write);
                        if let Err(error) = write_output {
                            eprintln!("Unable to write to output file: {}", error.kind());
                            sender.send(Message::Error(n)).unwrap();
                            return Err(n);
                        }
                    }
                    Err(_) => {
                        sender.send(Message::Error(n)).unwrap();
                        return Err(n);
                    }
                }
                match sender.send(Message::Progress) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(n),
                }
            });
            let mut file = file.lock().unwrap();
            if let Err(error) = file.flush() {
                error::download_output_error(error);
            }
        } else {
            let error = file.unwrap_err();
            error::download_output_error(error);
        }
        sender.send(Message::Done).unwrap();
    });

    loop {
        select! {
            recv(&receiver) -> message => match message.unwrap() {
                Message::Progress => {
                    progress += 1;
                    progress_bar.set_position(progress);
                },
                Message::Done => {
                    progress_bar.finish_with_message("downloaded");
                    break;
                },
                Message::Error(n) => {
                    progress_bar.abandon_with_message("⚠️");
                    error::download_error(n);
                    break;
                }
            }
        }
    }

    thread.join().unwrap();
}

use crate::config::Config;
use crate::config::Resume;
use crate::error;
use crate::password;
use bitvec::bitbox;
use crossbeam_channel::{bounded, select};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::ffi::OsString;
use std::io::Write;
use std::path::Path;
use std::sync::RwLock;
use std::sync::{Arc, Mutex};
use std::thread;

const HIBP_TOTAL: u64 = 16u64.pow(5) - 1;

#[derive(Copy, Clone)]
enum Message {
    Progress,
    Done,
    Error(u64),
}

pub fn downloader(output: OsString, force: bool, resume_status: bool) {
    let (sender, receiver) = bounded::<Message>(128);
    let sender = Arc::new(sender);
    let progress_bar = ProgressBar::new(HIBP_TOTAL);
    let mut progress = 0;
    let output_file = output.clone();

    if rayon::ThreadPoolBuilder::new()
        .num_threads(6)
        .build_global()
        .is_err()
    {
        eprintln!("Could not configure parallel downloading");
        return;
    }

    progress_bar.set_style(
        ProgressStyle::template(
            ProgressStyle::default_bar(),
            "[{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>7}/{len:7} ({eta})",
        )
        .progress_chars("#>-"),
    );

    let config = Config::load();
    let resume_file = if resume_status {
        let resume = config.resume_token.unwrap();
        Arc::new(RwLock::new(resume.resume))
    } else {
        Arc::new(RwLock::new(bitbox![0;HIBP_TOTAL as usize + 1]))
    };

    let resume = Arc::clone(&resume_file);

    let thread = thread::spawn(move || {
        let file = std::fs::File::options()
            .write(true)
            .create(true)
            .create_new(!force && !resume_status)
            .truncate(!resume_status)
            .open(output_file);

        if let Ok(file) = file {
            let file = Mutex::new(std::io::BufWriter::new(file));
            let agent = ureq::agent();
            let resume = Arc::clone(&resume);

            _ = (0..=HIBP_TOTAL).into_par_iter().try_for_each(|n| {
                {
                    let status = resume.read().unwrap();
                    if *status.get(n as usize).unwrap() {
                        sender.send(Message::Progress).unwrap();
                        return Ok(());
                    }
                }
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
                let mut token = resume.write().unwrap();
                token.get_mut(n as usize).unwrap().set(true);
                drop(token);
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
                    let mut config = Config::load();
                    config.resume_token = None;
                    config.store();
                    break;
                },
                Message::Error(n) => {
                    progress_bar.abandon_with_message("⚠️");
                    let mut config = Config::load();
                    let output: &Path = output.as_ref();
                    let output = output.canonicalize().unwrap().as_os_str().to_owned();
                    config.resume_token = Some(Resume {
                        resume: resume_file.read().unwrap().clone(),
                        download_file: output,
                    });
                    config.store();
                    error::download_error(n);
                    eprintln!("You can resume the download by running haveibeenpwned download-resume");
                    break;
                }
            }
        }
    }

    thread.join().unwrap();
}

use std::io::Write;

/*
 * here where the brute-force will happen
 * [JOB]
 * - define brute_force() function
 *
 * [WHAT DO I KNOW]
 * the brute_force() function will take a vector and url then return a string of all the result that made by the process
 * of sending requests using communecator::bring_status() function
 *
 * [WHY?]
 * this is the core of the tool, it will ues the whole functionalities to performe a directory
 * busting
*/
use crate::debug;
use super::communecator;
//use indicatif;
use owo_colors::{colors::Cyan, OwoColorize};

pub enum Mode {
    All,
    Hide
}

pub struct Progress {
    pos: u64,
    len: u64
}

pub async fn brute_force(url: String, paths: Vec<String>, mode: Mode) {
    //let mut list = Vec::new();
    debug::wait("checking for directories".to_string());
    let mut progress = Progress {
        pos: 0,
        len: paths.len().try_into().unwrap()
    };
    for i in 0..paths.len() {
        let path = paths.get(i);
        let path = match path {
            Some(x) => x,
            None => panic!("problem with the wordlist")
        };

        let status = communecator::bring_status(&url, &format!("/{}", path).to_string()).await;

        progress.pos += 1;
        let progress_format = format!("[{}/{}]", progress.pos, progress.len);

        match status {
            communecator::ResStatus::Found(x) => debug::confirm(format!("[{}] {}: {} {}\n",
                    path,
                    x.code,
                    x.state,
                    progress_format.fg::<Cyan>())),
            communecator::ResStatus::NotFound(x) => {
                match mode {
                    Mode::All => debug::deny(format!("[{}] {}: {} {}\n",
                            path,
                            x.code,
                            x.state,
                            progress_format.fg::<Cyan>())),
                    _ => {
                        print!("\x1B[1A\x1B[2K\r");
                        std::io::stdout().flush().unwrap();
                        debug::note(format!("{}", progress_format.fg::<Cyan>()));
                        continue;
                    }
                }
            }
        }
    }
    //debug::deny("checking for directories".to_string());
    //for i in 0..paths.len() {
    //    let path = paths.get(i).unwrap();
    //    let mut item = format!("({}) => ", path);
    //    item += communecator::bring_status(url.clone(), path.to_string())
    //        .await
    //        .readable();
    //    let status = communecator::bring_status(url.clone(), path.to_string())
    //        .await
    //        .kind();
    //    match status {
    //        communecator::StatusKind::Found => debug::confirm(item.clone()),
    //        communecator::StatusKind::NotFound => {
    //            match mode {
    //                Mode::All =>  debug::confirm(item.clone()),
    //                Mode::Hide => continue
    //            }
    //        }
    //    }
    //    list.push(item);
    //}
    //list
}

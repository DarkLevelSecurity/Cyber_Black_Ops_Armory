extern crate deep_buster;

use deep_buster::debug;
use deep_buster::buster::*;
//use reqwest;
//use tokio;
use argh::FromArgs;

struct BasicInputs {
    url: String,
    wl_file: String,
    wl_body: Vec<String>,
}

#[derive(FromArgs)]
#[argh(description = "scan for the web app/site directories")]
#[argh(help_triggers("-h", "--help"))]
struct Arguments {
    #[argh(option, short = 'u')]
    #[argh(description = "the url to scan")]
    url: Option<String>,

    #[argh(option, short = 'w')]
    #[argh(description = "the file that will be the wordlist")]
    wordlist: Option<String>,

    #[argh(switch)]
    #[argh(description = "show the found and not-found results [by default it only shows the found results]")]
    all: bool
}

fn main() {
    //let arguments = env::args().collect();
    let arguments: Arguments = argh::from_env();

    let mut inputs = BasicInputs {
        url: "".to_string(),
        wl_file: "".to_string(),
        wl_body: vec!["".to_string()]
    };

    let url = arguments.url; // URL
    inputs.url = match url {
        Some(x) => x,
        None => {
            debug::err("You need to give us a url\ntry '--help' or '-h'".to_string());
            return;
        }
    };
    let file = arguments.wordlist; // WORDLIST
    inputs.wl_file = match file {
        Some(x) => x,
        None => {
            debug::err("Pick a wordlist\ntry '--help' or '-h'".to_string());
            return;
        }
    };
    let mode = arguments.all; // MODE
    let mode: attack::Mode = {
        if mode {
            attack::Mode::All
        } else {
            attack::Mode::Hide
        }
    };
    let file_body = files_handling::get_wordlist(inputs.wl_file); // FILE-BODY
    inputs.wl_body = match file_body {
        Ok(x) => x,
        Err(x) => {
            debug::deny(x);
            return;
        }
    };


    let wordlist = inputs.wl_body;
    let rt = tokio::runtime::Runtime::new().unwrap();
    let future = 
        attack::brute_force(inputs.url, wordlist, mode);

    debug::head("result".to_string());
    rt.block_on(future);
}

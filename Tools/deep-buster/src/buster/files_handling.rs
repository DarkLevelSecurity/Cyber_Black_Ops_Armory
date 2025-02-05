/*
 * here we will pick a wordlists from files.txt
 * [JOB]
 * - a get_wordlist() function that returns a vector for the whole file lines
 * 
 * [WHAT DO I KNOW]
 * it will open a file then read it then it will return the value
 *
 * [WHY?]
 * to make the hacker able to attach his own wordlists
*/

use std::fs;
use crate::debug;

pub fn get_wordlist(path: String) -> Result<Vec<String>, String> {
    //let mut f = File::open(path)
    //    .expect("Can't reach the wordlist file");
    debug::wait("reading file".to_string());
    let body = fs::read_to_string(path.as_str());
    let body = match body {
        Ok(x) => x,
        Err(_x) => return Result::Err("Couldn't read the file".to_string())
    };

    debug::confirm("read the file :)".to_string());
    let mut list: Vec<String> = Vec::new();
    for i in body.lines() {
        list.push(i.to_string());
    }

    Result::Ok(list)
}

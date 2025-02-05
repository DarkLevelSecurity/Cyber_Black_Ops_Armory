# DIR-BUSTER
Project: [dir-buster]
by: [Red Hunter] 
start-date: [2025/1/29]

### Overview
A penetration testing tool aimed at scanning directories in a website's path to map the site and expand the the scope using Rust, with a focus on speed, security, and deep scanning.

### Feature
1. directories brute-forcing
2. in-depth directories brute-forcing
3. extract results in JSON file
4. built-in wordlists
supported systems: [Linux]
Client-view: [CLI]

### Technologies
Language: [RUST]
Libraries and Frameworks:
- [reqwest]
- tokio

### Project Structure
```txt
src 
|- main.rs
|- lib.rs 
|- buster/
|  |- mod.rs # main structures and functions
|  |- communecator.rs # sending requests and handling respones features
|  |- files_handling.rs # extract data from files such a wordlists
|  |- attack.rs # where the brute-force will happen?
|- debug/
|  |- mod.rs # debuging messages
|- Cargo.toml
|- Cargo.lock
```

### Usage
just go inside the project directory an run
```
cargo build
```
to build the tool, and you can run `target/debug/dir-buster`

`Usage: dir-buster -u <url> -w <wordlist> -d* <depth> --hide* --all*`
-x* = -x is optional (use -h for a manual)

### How it works?
it sends requests to the path and get the respones status code and based on it; it will show results.
and you can control the output flow using a flags such `--hide`, you also can ask for deep enumeration which makes an enum to not only the path but also the other found directories (it takes time but gives you a bigger view)

# The End 
feel free to steal the code and improve it the way you like, I would be grateful if you messaged me and showed me your own tool built on this project

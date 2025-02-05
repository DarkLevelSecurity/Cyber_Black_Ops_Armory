/*
 * for this file i will make the next shortcuts to help me debuging an formating the output
 * - confirm(x) => "[+] x"
 * - deny(x) => "[-] x"
 * - err(x) => "[!] ERROR: x"
 * - warn(x) => "[!] x"
 * - input(x) => "[?] x: "
 * - wait(x) => "[^] x..."
 * - note(x) => "[*] x"
 * - head(x) => "[=======(x)=======]"
*/

use owo_colors::{colors::{css::Purple, Blue, Green, Red}, OwoColorize}; 

pub fn confirm(msg: String) {println!("{} {}", "[+]".fg::<Green>(), msg)}
pub fn deny(x: String) {println!("{} {}", "[-]".fg::<Purple>(), x)}
pub fn err(x: String) {println!("{} ERROR: {}", "[!]".fg::<Red>(), x)}
pub fn warn(x: String) {println!("{} {}", "[!]".fg::<Red>(),  x)}
pub fn input(x: String) {println!("[?] {}:", x)}
pub fn wait(x: String) {println!("[^] {}...", x)}
pub fn note(x: String) {println!("[*] {}", x)}
pub fn head(x: String) {println!("[=======({})=======]", x.to_uppercase().fg::<Blue>())}

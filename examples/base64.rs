extern crate captcha;

use captcha::{gen, Difficulty};
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let s = gen(Difficulty::Easy).as_base64().expect("Error.");
    let mut f = File::create("/tmp/captcha_base64.txt")?;
    f.write_all(s.as_bytes())
}

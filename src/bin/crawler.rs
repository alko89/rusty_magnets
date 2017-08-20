extern crate curl;

use std::io::{stdout, Write};

use curl::easy::Easy;

fn main() {
    let mut easy = Easy::new();
    easy.url("https://thepiratebay.org/").unwrap();
    easy.write_function(|data| {
        Ok(stdout().write(data).unwrap())
    }).unwrap();
    easy.perform().unwrap();

    let body = easy.response_code().unwrap();
    println!("{}", body);
}

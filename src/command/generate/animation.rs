use crate::command::generate::random_ascii;
use crossterm::style::Stylize;
use rand::prelude::SliceRandom;
use std::io::Write;
use std::time::Duration;
use std::{io, thread};

const PLACEHOLDER_CHAR: u8 = b'\0';
pub(crate) fn animate(pass: &str) {
    let len = pass.len();
    let mut state = vec![PLACEHOLDER_CHAR; len];
    while let Some(&i) = missing_indexes(&state).choose(&mut rand::thread_rng()) {
        advance_and_print_state(pass, &mut state, i);
    }
}

fn advance_and_print_state(pass: &str, state: &mut [u8], i: usize) {
    state[i] = pass.as_bytes()[i];
    let s = state.iter().fold(String::new(), |mut acc, &elem| {
        if elem == PLACEHOLDER_CHAR {
            acc.push(random_ascii() as char);
        } else {
            acc.push_str(format!("{}", (elem as char).bold()).as_str());
        }
        acc
    });
    print!("\r{s}");
    io::stdout().flush().unwrap();
    thread::sleep(Duration::from_millis((2000 / pass.len()) as u64));
}

fn missing_indexes(state: &[u8]) -> Vec<usize> {
    state
        .iter()
        .enumerate()
        .filter_map(|(i, c)| {
            if *c == PLACEHOLDER_CHAR {
                Some(i)
            } else {
                None
            }
        })
        .collect()
}

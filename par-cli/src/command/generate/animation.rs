use crossterm::style::Stylize;
use par_core;
use rand::prelude::SliceRandom;
use std::cmp::min;
use std::io::Write;
use std::time::Duration;
use std::{io, thread};

pub(crate) fn animate(pass: &str) {
    let len = pass.len();
    let mut state: Vec<Option<char>> = vec![None; len];
    while let Some(&i) = missing_indexes(&state).choose(&mut rand::thread_rng()) {
        advance_and_print_state(pass, &mut state, i);
    }
}

fn advance_and_print_state(pass: &str, state: &mut [Option<char>], i: usize) {
    state[i] = pass.chars().nth(i);
    let s = state.iter().fold(String::new(), |mut acc, &elem| {
        if let Some(char) = elem {
            acc.push_str(format!("{}", char.bold()).as_str());
        } else {
            acc.push(par_core::random_ascii());
        }
        acc
    });
    print!("\r{s}");
    io::stdout().flush().unwrap();
    thread::sleep(Duration::from_millis(min(50, 2000 / pass.len()) as u64));
}

fn missing_indexes(state: &[Option<char>]) -> Vec<usize> {
    state
        .iter()
        .enumerate()
        .filter_map(|(i, c)| if c.is_none() { Some(i) } else { None })
        .collect()
}

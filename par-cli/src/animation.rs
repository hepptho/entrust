use color_print::cformat;
use rand::prelude::SliceRandom;
use std::cmp::min;
use std::io::Write;
use std::time::Duration;
use std::{io, thread};

pub(crate) fn animate(pass: &str) {
    let len = pass.len();
    let mut state: Vec<Option<char>> = vec![None; len];
    let mut buf = String::with_capacity(len * 10);
    for _ in 0..5 {
        print_state(&state, &mut buf);
    }
    while let Some(&i) = missing_indexes(&state).choose(&mut rand::thread_rng()) {
        state[i] = pass.chars().nth(i);
        print_state(&state, &mut buf);
    }
}

fn print_state(state: &[Option<char>], buf: &mut String) {
    buf.clear();
    for elem in state {
        if let Some(char) = elem {
            buf.push_str(cformat!("<bold>{char}</>").as_str());
        } else {
            buf.push(par_core::random_ascii());
        }
    }
    print!("\r{buf}");
    io::stdout().flush().unwrap();
    thread::sleep(Duration::from_millis(min(50, 2000 / state.len()) as u64));
}

fn missing_indexes(state: &[Option<char>]) -> Vec<usize> {
    state
        .iter()
        .enumerate()
        .filter_map(|(i, c)| if c.is_none() { Some(i) } else { None })
        .collect()
}

#![allow(dead_code)]

const STRING: &str =
    "Never perfect. Perfection goal that changes. Never stops moving. Can chase. Cannot Catch.";
const CHARS: [u8; 57] = *b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ .,!?";
const GUES_BATCH_SIZE: usize = 50;
const BAR_WIDTH: usize = 20;
const FRAME_DELAY: Duration = Duration::from_millis(800);

use std::fmt::Display;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use minifb::Key::G;
use rand::seq::IndexedRandom;

pub fn genetic_loop() {
    let input = STRING;
    let evolution = Evolution::new(input);
    evolution.commense();
}

struct Evolution {
    input: String,
}

impl Evolution {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
        }
    }

    fn commense(&self) -> Gues {
        let best_guesses = self.get_initial_best_guesses();
        self.iterate(&best_guesses)
    }

    fn iterate(&self, initial_best_guesses: &Vec<Gues>) -> Gues {
        let mut merged_guesses = initial_best_guesses.clone();
        let mut progress = Progress::new(&self.input);
        loop {
            merged_guesses = self.merge_best_guesses(&merged_guesses);

            let best_gues = merged_guesses
                .iter()
                .max_by_key(|g| g.correct_indices.len())
                .unwrap();

            progress.show(best_gues);

            if best_gues.correct_indices.len() == self.input.len() {
                return best_gues.clone();
            }
        }
    }

    fn merge_best_guesses(&self, survivors: &Vec<Gues>) -> Vec<Gues> {
        let mut merged_guesses: Vec<Gues> = Vec::new();
        loop {
            let new_gues = self.evolve_mutate_improve(survivors);
            merged_guesses.push(new_gues);
            if merged_guesses.len() == GUES_BATCH_SIZE {
                break merged_guesses;
            }
        }
    }

    fn evolve_mutate_improve(&self, subjects: &Vec<Gues>) -> Gues {
        let picked: Vec<&Gues> = subjects.sample(&mut rand::rng(), 2).collect();
        let mut educated_gues = Gues::create_random_string_as_bytes(picked[0].gues.len());
        let a = picked[0];
        let b = picked[1];
        for &i in &a.correct_indices {
            educated_gues[i] = a.gues[i];
        }

        for &i in &b.correct_indices {
            educated_gues[i] = b.gues[i];
        }

        Gues::educated_gues(&self.input, &educated_gues)
    }

    fn get_initial_best_guesses(&self) -> Vec<Gues> {
        loop {
            let guesses = get_scored_guesses(&self.input);
            let filtered_guesses: Vec<Gues> = guesses
                .into_iter()
                .filter(|g| !g.correct_indices.is_empty())
                .collect();

            if filtered_guesses.is_empty() {
                continue;
            }

            break filtered_guesses;
        }
    }
}

fn get_scored_guesses(input: &str) -> Vec<Gues> {
    let mut guesses = Vec::new();
    for _ in 0..GUES_BATCH_SIZE {
        guesses.push(Gues::new(input));
    }
    guesses
}

#[derive(Clone)]
struct Gues {
    gues: Vec<u8>,
    correct_indices: Vec<usize>,
}

impl Gues {
    pub fn educated_gues(input: &str, educated_gues: &[u8]) -> Self {
        let correct_indices = Gues::get_correct_indices(input, &educated_gues);
        Self {
            gues: educated_gues.to_vec(),
            correct_indices,
        }
    }

    pub fn new(input: &str) -> Self {
        let gues = Gues::create_random_string_as_bytes(input.len());
        Gues::educated_gues(input, &gues)
    }

    fn get_correct_indices(input: &str, gues: &[u8]) -> Vec<usize> {
        input
            .as_bytes()
            .iter()
            .zip(gues)
            .enumerate()
            .filter(|(_, (a, b))| a == b)
            .map(|t| t.0)
            .collect()
    }

    /// Correct chars in green, wrong ones dimmed
    fn colored(&self) -> String {
        self.gues
            .iter()
            .enumerate()
            .map(|(i, &c)| {
                let c = c as char;
                if self.correct_indices.contains(&i) {
                    format!("\x1b[32m{c}\x1b[0m")
                } else {
                    format!("\x1b[2m{c}\x1b[0m")
                }
            })
            .collect()
    }

    pub fn create_random_string_as_bytes(length: usize) -> Vec<u8> {
        (0..length)
            .map(|_| CHARS[rand::random_range(0..CHARS.len())] as u8)
            .collect()
    }
}

impl Display for Gues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.gues))
    }
}

/// Prints one live-updating status line per generation,
/// only keeping the line when the best score improved
struct Progress {
    total: usize,
    generation: usize,
    best_score: usize,
}

impl Progress {
    fn new(target: &str) -> Self {
        // Padded so the target lines up with the guesses in the status lines
        println!("{:<39}\x1b[1m{}\x1b[0m\n", "target", target);
        Self {
            total: target.len(),
            generation: 0,
            best_score: 0,
        }
    }

    fn show(&mut self, best_gues: &Gues) {
        self.generation += 1;
        let score = best_gues.correct_indices.len();

        // Overwrite the current line; keep it (newline) only when the score improved
        print!("\r\x1b[2K{}", self.status_line(best_gues));
        if score > self.best_score {
            self.best_score = score;
            println!();
        }
        io::stdout().flush().unwrap();

        if score < self.total {
            thread::sleep(FRAME_DELAY);
        }
    }

    fn status_line(&self, gues: &Gues) -> String {
        let score = gues.correct_indices.len();
        let filled = score * BAR_WIDTH / self.total;
        let bar = "█".repeat(filled) + &"░".repeat(BAR_WIDTH - filled);
        format!(
            "gen {:>4} [{bar}] {score:>2}/{}  {}",
            self.generation,
            self.total,
            gues.colored()
        )
    }
}

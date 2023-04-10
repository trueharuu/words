#![warn(clippy::all)]
#![allow(clippy::suspicious_else_formatting)]
use std::time::Instant;

use clap::Parser;
use owo_colors::{AnsiColors, DynColor, OwoColorize};
use words::words;

use crate::words::small_words;
mod words;
fn main() {
    let cli = Args::parse();
    // let mwords = missing_words();
    let swords = small_words();

    let n = Instant::now();
    let mut word = search_words(
        cli.word.clone(),
        cli.list
            .map(|x| x.split_ascii_whitespace().map(str::to_owned).collect())
            .unwrap_or_else(words),
        cli.min_length,
    );
    word.sort();
    word.sort_by_key(|x| x.len());
    let a = n.elapsed();

    println!(
        "found {} words from {} in {}s:\n\n{}",
        word.len(),
        cli.word.yellow().bold(),
        color_on_scale(
            a.as_secs_f64(),
            [
                (AnsiColors::Green, 0.0),
                (AnsiColors::Yellow, 5.0),
                (AnsiColors::Red, 10.0)
            ]
        ),
        if cli.quiet {
            String::new()
        } else {
            word.chunks(15)
                .map(|x| {
                    x.iter()
                        .map(|x| {
                            if x == &cli.word {
                                x.green().to_string()
                            }
                            // else if mwords.contains(&x.to_lowercase()) {
                            //     x.black().to_string()
                            // }
                            else if swords.contains(&x.to_lowercase()) {
                                x.yellow().to_string()
                            } else {
                                x.black().to_string()
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(" ")
                })
                .collect::<Vec<_>>()
                .join("\n")
        }
    )
}

fn color_on_scale<const N: usize>(n: f64, colors: [(impl DynColor + Clone, f64); N]) -> String {
    use owo_colors::style;

    let lowest = colors.iter().rfind(|x| n >= x.1);

    format!("{}", n.style(style().color(lowest.unwrap().0.clone())))
}

fn search_words(word: String, mut list: Vec<String>, min_length: u8) -> Vec<String> {
    // table of u8, 5 bits per char - 97 (0..26), 3 bits for stored frequency
    let mut cc: [u8; 26] = [
        0, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120, 128, 136, 144, 152, 160,
        168, 176, 184, 192, 200,
    ];

    for c in word.chars() {
        cc[c as usize - 97] += 1;
    }

    list.retain(|x| {
        if x.len() > word.len() || (x.len() as u8) < min_length {
            return false;
        }

        let mut nc = cc;
        let mut i = 0;
        for c in x.bytes() {
            if (nc[c as usize - 97] & 0x7) != 0 {
                nc[c as usize - 97] -= 1;
                i += 1;
            } else {
                break;
            }
        }

        i == x.len()
    });

    list
}

#[derive(clap::Parser, Debug)]
#[command(author, version)]
pub struct Args {
    word: String,
    #[arg(short, long)]
    list: Option<String>,
    #[arg(short, long, default_value_t = 3)]
    min_length: u8,
    #[arg(short, long, default_value_t = false)]
    quiet: bool,
}

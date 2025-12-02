use std::{fs::File, io::BufReader, path::PathBuf};

use clap::Parser;
use colorize::AnsiColor;
use seq_macro::seq;

seq!(I in 1..=2 {
    mod day~I;
});

type Input = BufReader<File>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    day: u32,
    #[arg(short, long)]
    test: bool,
    #[arg(short, long)]
    input: Option<String>,
    #[clap(short = '1', long = "part1", overrides_with = "part2")]
    part1: bool,
    #[clap(short = '2', long = "part2")]
    part2: bool,
}

fn main() {
    let args = Args::parse();

    let mut path = PathBuf::from("./inputs");
    path.push(args.day.to_string());
    path.push(match &args.input {
        Some(s) => s,
        None => {
            if args.test {
                "test.txt"
            } else {
                "input.txt"
            }
        }
    });

    let open_file = || BufReader::new(File::open(&path).expect("Input file cannot be opened!"));

    seq!(I in 1..=2 {
        if args.day == I {
            if !args.part2 {
                println!("{} {}", "Part 1:".bold().blue(), day~I::part1(open_file()));
            }
            if !args.part1 {
                println!("{} {}", "Part 2:".bold().magenta(),  day~I::part2(open_file()));
            }
        }
    });
}

use std::{fs::File, io::BufReader, path::PathBuf, time::Instant};

use clap::Parser;
use colorize::AnsiColor;
use seq_macro::seq;

seq!(I in 1..=4 {
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

    macro_rules! run {
        ($fn:expr, $other:ident, $msg:expr, $colour:ident) => {
            if !args.$other {
                let start = Instant::now();
                let output = $fn(open_file());
                let duration = Instant::now().duration_since(start);
                println!(
                    "{time} {msg} {output}",
                    time = format!("[{:>8?}]", duration).b_black(),
                    msg = $msg.bold().$colour(),
                    output = output
                );
            }
        };
    }

    seq!(I in 1..=4 {
        if args.day == I {
            println!("{}", format!("### Day {} ###", I).bold().green());
            run!(day~I::part1, part2, "Part 1:", blue);
            run!(day~I::part2, part1, "Part 2:", magenta);
        }
    });
}

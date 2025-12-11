use std::{fs::File, io::BufReader, path::PathBuf, time::Instant};

use anyhow::Result;
use clap::Parser;
use colorize::AnsiColor;
use seq_macro::seq;

seq!(I in 1..=11 {
    mod day~I;
});

type Input = BufReader<File>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    day: Option<u32>,
    #[arg(short, long)]
    test: bool,
    #[arg(short, long)]
    input: Option<String>,
    #[clap(short = '1', long = "part1", overrides_with = "part2")]
    part1: bool,
    #[clap(short = '2', long = "part2")]
    part2: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    macro_rules! run {
        ($fn:expr, $path:expr, $msg:expr, $colour:ident) => {
            let start = Instant::now();
            let output = $fn(BufReader::new(
                File::open($path).expect("Input file cannot be opened!"),
            ))?;
            let duration = Instant::now().duration_since(start);
            println!(
                "{time} {msg} {output}",
                time = format!("[{:>10?}]", duration).b_black(),
                msg = $msg.bold().$colour(),
                output = output
            );
        };
    }

    seq!(I in 1..=11 {
        match args.day {
            Some(I) | None => {
                let mut path = PathBuf::from("./inputs");
                path.push(I.to_string());
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
                println!("{}", format!("### Day {} ###", I).bold().green());
                if !args.part2 {
                    run!(day~I::part1, &path, "Part 1:", blue);
                }
                if !args.part1 {
                    run!(day~I::part2, &path, "Part 2:", magenta);
                }
            },
            _ => ()
        }
    });

    Ok(())
}

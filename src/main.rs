use ferris_says::say;
use std::io::{stdout, BufWriter};
use structopt::StructOpt;


fn main() {
    let input = std::env::args().nth(1).expect("no expression provided");
    let args = Cli {
        input,
    };

    let stdout = stdout();
    let width = args.input.len() + 2;

    let mut writer = BufWriter::new(stdout.lock());
    say(args.input.as_ref(), width, &mut writer).unwrap();
}

#[derive(StructOpt)]
struct Cli {
    input: String,
}
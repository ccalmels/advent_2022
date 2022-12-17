use clap::Parser;
mod days;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(trailing_var_arg = true)]
    days: Vec<u32>,
}

fn main() {
    let args = Args::parse();

    if args.days.is_empty() {
        advent_2022::resolve_all();
    } else {
        for d in args.days {
            advent_2022::resolve_one(d);
        }
    }
}

use clap::Parser;
#[derive(Parser)]
struct Args {
    city: String,
}
fn main() {
    let args = Args::parse();
    println!("{}", args.city);
}

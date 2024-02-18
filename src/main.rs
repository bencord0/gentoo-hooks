use clap::{Parser};

#[derive(Parser)]
struct Opt {
    #[arg(long)]
    function: String,
}

fn main() {
    let opt = Opt::parse();
    println!("portage hook function: {}", opt.function);
}

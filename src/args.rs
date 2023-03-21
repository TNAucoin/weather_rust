use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    // Name of the city to check weather
    #[arg(short, long)]
    pub zip: u64,
}

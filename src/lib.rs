use ::clap;
use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[clap(default_value_t = 3)]
    pub rows: usize,

    #[clap(default_value_t = 3)]
    pub columns: usize,
}

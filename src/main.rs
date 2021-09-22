mod helpers;
mod least_squares;

use structopt::StructOpt;

use least_squares::find_best_line;
use helpers::load_dataset;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short, long, default_value = "./dataset.csv")]
    dataset: String,
}

fn main() {
    let args = Cli::from_args();

    match load_dataset(&args.dataset) {
        Ok(points) => {
            match find_best_line(&points) {
                Ok(line) => print!("Best line is defined by: y = {:?} * x + {:?}\n", line.slope, line.intersect),
                Err(e) => print!("{}\n", e)
            }
        },
        Err(e) => print!("{}\n", e)
    }
}

use color_eyre::{eyre::Report, eyre::Result};
use std::time::Instant;

mod solver;

fn main() -> Result<(), Report> {
    color_eyre::install()?;

    let start = Instant::now();

    solver::solve()?;

    let duration = start.elapsed();
    println!("Time elapsed in solving is: {:?}", duration);

    Ok(())
}

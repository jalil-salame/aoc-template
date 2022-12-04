use color_eyre::{Result, Report};

#[allow(dead_code)]
fn empty_option_err() -> Report {
    use std::io::{Error, ErrorKind::Other};
    Error::new(Other, "Option was empty").into()
}

static INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    color_eyre::install()?;

    let p1_solution = "Nothing Yet";
    let p2_solution = "Nothing Yet";

    println!("Problem 1: {p1_solution}\nProblem 2: {p2_solution}");

    Ok(())
}

use color_eyre::{Report, Result};
use std::time::Instant;

#[allow(dead_code)]
fn empty_option_err() -> Report {
    use std::io::{Error, ErrorKind::Other};
    Error::new(Other, "Option was empty").into()
}

static INPUT: &str = include_str!("input");

fn main() -> Result<()> {
    color_eyre::install()?;

    let now = Instant::now();

    let _ = {
        let now = Instant::now();

        let data = parse::input(INPUT)?.1;

        println!("Parsing took:    {:>16?}", now.elapsed());
        data
    };

    let now_ = Instant::now();

    // Add common data structures to Problem 1 and Problem 2 here

    println!("Processing took: {:>16?}", now_.elapsed());

    let problem_1_solution = {
        let now = Instant::now();

        let solution = "Nothing yet";

        println!("Problem 1 took:  {:>16?}", now.elapsed())
        solution
    };

    let problem_2_solution = {
        let now = Instant::now();

        let solution = "Nothing yet";

        println!("Problem 2 took:  {:>16?}", now.elapsed())
        solution
    };

    println!("Total runtime:   {:>16?}", now.elapsed());
    println!("----------------O----------------");
    println!("Problem 1:       {problem_1_solution:>16}");
    println!("Problem 2:       {problem_2_solution:>16}");

    Ok(())
}

mod parse {
    use nom::IResult;

    pub fn input(input: &str) -> IResult<&str, ((),)> {
        todo!("Implement parsing of {input}") // terminated((), eof)(input)
    }
}

#[cfg(test)]
mod test {
    use crate::parse;
    use color_eyre::Result;
    #[allow(unused)]
    use pretty_assertions::{assert_eq, assert_ne};

    static INPUT: &str = include_str!("test_input");

    #[test]
    fn parse_test_input() -> Result<()> {
        let (rest, data) = parse::input(INPUT)?;
        assert_eq!(rest, "");
        assert_eq!(data, ((),));

        Ok(())
    }

    #[test]
    fn parse_input() -> Result<()> {
        let (rest, data) = parse::input(crate::INPUT)?;

        assert_eq!(rest, "");
        assert_eq!(data, ((),));

        Ok(())
    }

    #[test]
    fn problem_1() -> Result<()> {
        let data = parse::input(INPUT)?.1;

        assert_eq!(data, ((),));

        Ok(())
    }

    #[test]
    fn problem_2() -> Result<()> {
        let data = parse::input(INPUT)?.1;

        assert_eq!(data, ((),));

        Ok(())
    }
}

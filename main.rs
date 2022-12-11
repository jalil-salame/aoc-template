use color_eyre::{Report, Result};

#[allow(dead_code)]
fn empty_option_err() -> Report {
    use std::io::{Error, ErrorKind::Other};
    Error::new(Other, "Option was empty").into()
}

static INPUT: &str = include_str!("input");

fn main() -> Result<()> {
    color_eyre::install()?;

    let data = parse::input(INPUT)?;
    println!("{data:?}");

    let p1_solution = "Nothing Yet";
    let p2_solution = "Nothing Yet";

    println!("Problem 1: {p1_solution}\nProblem 2: {p2_solution}");

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

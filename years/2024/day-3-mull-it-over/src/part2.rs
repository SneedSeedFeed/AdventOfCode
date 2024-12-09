use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_till, take_while},
    combinator::{map_res, value},
    error::Error,
    multi::fold_many0,
    sequence::pair,
    IResult,
};

#[derive(Debug, Clone, Copy)]
enum Part2Fragment {
    Do,
    Dont,
    Mul { left: i64, right: i64 },
    Gibberish,
}

// Maybe it's against the spirit of things, but I want to use nom here because I love nom (unless they reject my PR then I hate them)
pub fn part_two(bytes: &[u8]) -> i64 {
    fold_many0(
        parse_fragment,
        || (true, 0i64),
        |(mut do_mul, mut sum), fragment| {
            match fragment {
                Part2Fragment::Do => do_mul = true,
                Part2Fragment::Dont => do_mul = false,
                Part2Fragment::Mul { left, right } => {
                    if do_mul {
                        sum += left * right
                    }
                }
                Part2Fragment::Gibberish => {}
            };
            (do_mul, sum)
        },
    )(bytes)
    .unwrap()
    .1
     .1
}

fn parse_fragment(input: &[u8]) -> IResult<&[u8], Part2Fragment, Error<&[u8]>> {
    alt((
        value(Part2Fragment::Do, tag(b"do()")),
        value(Part2Fragment::Dont, tag(b"don't()")),
        parse_mul_op,
        value(
            Part2Fragment::Gibberish,
            pair(
                take(1usize), // make sure the parser actually moves forwards
                take_till(|b| b == b'm' || b == b'd'),
            ),
        ),
    ))(input)
}

// Bad nom code but we ball, it's readable
fn parse_mul_op(input: &[u8]) -> IResult<&[u8], Part2Fragment, Error<&[u8]>> {
    let (rem, _mul_tag) = tag(b"mul(")(input)?;
    let (rem, left) = parse_i64(rem)?;
    let (rem, _comma) = tag(b",")(rem)?;
    let (rem, right) = parse_i64(rem)?;
    let (rem, _r_paren) = tag(b")")(rem)?;
    Ok((rem, Part2Fragment::Mul { left, right }))
}

fn parse_i64(input: &[u8]) -> IResult<&[u8], i64, Error<&[u8]>> {
    map_res(take_while(|b: u8| b.is_ascii_digit()), |slice| {
        let as_utf8: Result<&str, Box<dyn std::error::Error>> =
            std::str::from_utf8(slice).map_err(Box::from);
        let as_utf8 = as_utf8?;

        let parse_res: Result<i64, Box<dyn std::error::Error>> =
            str::parse::<i64>(as_utf8).map_err(Box::from);
        parse_res
    })(input)
}

#[cfg(test)]
mod test {
    use super::part_two;

    #[test]
    fn test_given_input() {
        let input = b"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let part_2 = part_two(input);
        assert_eq!(part_2, 48)
    }
}

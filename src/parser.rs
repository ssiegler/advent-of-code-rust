use nom::character::complete::digit1;
use nom::combinator::map_opt;
use nom::IResult;

pub fn integer(input: &[u8]) -> IResult<&[u8], u32> {
    map_opt(digit1, atoi::atoi)(input)
}

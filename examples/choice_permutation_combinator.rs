use nom::character::complete::{alpha1, digit1};
use nom::branch::permutation;
use nom::IResult;
use nom::bytes::complete::tag;

fn parser(input: &str) -> IResult<&str, (&str, &str, &str)> {
    permutation(
        (tag("ab"), tag("cd"),tag("12"))
    )(input)
}

fn main() {
  let result = parser("12abcdc");
  println!("{:?}",result)
}
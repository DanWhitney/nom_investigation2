use nom::character::complete::{alpha1, digit1};
use nom::branch::alt;
use nom::IResult;

fn parser(input: &str) -> IResult<&str, &str> {
  alt((alpha1, digit1))(input)
}

fn main() {
  let result = parser("666777 jjk");
  println!("{:?}",result)
}
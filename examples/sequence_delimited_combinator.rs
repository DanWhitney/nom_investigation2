use nom::sequence::delimited;
use nom::bytes::complete::tag;
use nom::IResult;

fn delimited_parser<'a>(i: &'a str,first: &str, second: &str, third: &str) -> IResult<&'a str,&'a str> {
    delimited(tag(first), tag(second), tag(third))(i)
}

fn main () {
    let result = delimited_parser("(ab)cd", "(", "ab", ")");
    println!("{:?}", result);
}



use nom::sequence::terminated;
use nom::bytes::complete::tag;
use nom::IResult;

fn terminated_parser<'a>(i: &'a str,first: &str, second: &str) -> IResult<&'a str,&'a str> {
    terminated(tag(first), tag(second))(i)
}

fn main () {
    let result = terminated_parser("abXYZ", "ab", "XY");
    println!("{:?}", result);
}



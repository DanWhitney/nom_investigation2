use nom::sequence::pair;
use nom::bytes::complete::tag;
use nom::IResult;

fn pair_parser<'a>(i: &'a str,first: &str, second: &str) -> IResult<&'a str,(&'a str, &'a str)> {
    pair(tag(first), tag(second))(i)
}

fn main () {
    let result = pair_parser("abXYZ", "ab", "XY");
    println!("{:?}", result);
}



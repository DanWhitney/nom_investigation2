use nom::sequence::preceded;
use nom::bytes::complete::tag;
use nom::IResult;

fn preceded_parser<'a>(i: &'a str,first: &str, second: &str) -> IResult<&'a str,&'a str> {
    preceded(tag(first), tag(second))(i)
}

fn main () {
    let result = preceded_parser("abXYZ", "ab", "XY");
    println!("{:?}", result);
}



use nom::sequence::tuple;
use nom::bytes::complete::tag;
use nom::IResult;

fn tuple_parser<'a>(i: &'a str,first: &str, second: &str, third: &str) -> IResult<&'a str,(&'a str, &'a str, &'a str)> {
    tuple((tag(first), tag(second), tag(third)))(i)
}

fn main () {
    let result = tuple_parser("abXYZ!", "ab", "XY", "Z");
    println!("{:?}", result);
}



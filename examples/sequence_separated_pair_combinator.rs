use nom::sequence::separated_pair;
use nom::bytes::complete::tag;
use nom::IResult;

fn separated_pair_parser<'a>(i: &'a str,first: &str, second: &str, third: &str) -> IResult<&'a str,(&'a str, &'a str)> {
    separated_pair(tag(first), tag(second), tag(third))(i)
}

fn main () {
    let result = separated_pair_parser("Hello, World!", "Hello", ", ", "World");
    println!("{:?}", result);
}



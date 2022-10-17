use nom::multi::count;
use nom::bytes::complete::take;
use nom::IResult;

fn count_parser<'a>(i: &'a str) -> IResult<&'a str,Vec<&'a str>> {
    count(take(2u32),4usize)(i)
}

fn main () {
    let result = count_parser("abcdefg");
    println!("{:?}", result);
}



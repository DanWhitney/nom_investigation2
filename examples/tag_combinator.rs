use nom::IResult;
use nom::bytes::complete::tag;

fn tag_parser<'a>(i: &'a str,tag_part: &str) -> IResult<&'a str,&'a str> {
    tag(tag_part)(i)
}
fn main() {
    let result = tag_parser("Hello, World!","Hello").unwrap();
    println!("{:?}", result);
}
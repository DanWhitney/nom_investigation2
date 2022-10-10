use nom::IResult;
use nom::bytes::complete::tag_no_case;

fn tag_no_case_parser<'a>(i: &'a str,tag_part: &str) -> IResult<&'a str,&'a str> {
    tag_no_case(tag_part)(i)
}
fn main() {
    let result = tag_no_case_parser("Hello, World!","hello").unwrap();
    println!("{:?}", result);
}
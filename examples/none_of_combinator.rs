use nom::IResult;
use nom::character::complete::none_of;

fn none_of_parser(i: &str) -> IResult<&str,char> {
    none_of("abc")(i)
}
fn main() {
    let result = none_of_parser("xyabc").unwrap();
    println!("{:?}", result);
}
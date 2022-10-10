use nom::IResult;
use nom::character::complete::one_of;

fn one_of_parser(i: &str) -> IResult<&str,char> {
    one_of("abcd")(i)
}
fn main() {
    let result = one_of_parser("abc").unwrap();
    println!("{:?}", result);
}
use nom::IResult;
use nom::character::complete::char;

fn char_parser(i: &str) -> IResult<&str,char> {
    char('a')(i)
}
fn main() {
    let result = char_parser("abc").unwrap();
    println!("{:?}", result);
}
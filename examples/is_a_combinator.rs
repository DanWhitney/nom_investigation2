use nom::IResult;
use nom::bytes::complete::is_a;

fn hex(s: &str) -> IResult<&str, &str> {
  is_a("1234567890ABCDEF")(s)
}

fn main () {
    let result = hex("123EF and its as simple as that");
    println!("{:?}", result);
}
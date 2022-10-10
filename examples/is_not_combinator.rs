use nom::{bytes::complete::is_not, IResult};

fn not_space(s: &str) -> IResult<&str, &str> {
  is_not(" \t\r\n")(s)
}

fn main() {
    let result = not_space("Hello, World!").unwrap();
    println!("{:?}", result);
}
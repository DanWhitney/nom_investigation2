use nom::{IResult, bytes::complete::take_until, character:: is_alphabetic};


fn take_until_hello_parser(i: &str) -> IResult<&str,&str> {
    take_until("Hello")(i)
}
fn main() {
    let result = take_until_hello_parser("12345678910Hello, World!123").unwrap();
    println!("{:?}", result);
}
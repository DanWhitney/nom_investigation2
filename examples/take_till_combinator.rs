use nom::{IResult, bytes::complete::take_till, character:: is_alphabetic};


fn take_till_alphabetic_parser(i: &[u8]) -> IResult<&[u8],&[u8]> {
    take_till(is_alphabetic)(i)
}
fn main() {
    let result = take_till_alphabetic_parser(b"12345678910Hello, World!123").unwrap();
    println!("{:?}", result);
}
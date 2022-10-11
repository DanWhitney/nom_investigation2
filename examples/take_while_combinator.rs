use nom::{IResult, bytes::complete::take_while, character:: is_alphabetic};


fn take_while_alphabetic_parser(i: &[u8]) -> IResult<&[u8],&[u8]> {
    take_while(is_alphabetic)(i)
}
fn main() {
    let result = take_while_alphabetic_parser(b"Hello, World!123").unwrap();
    println!("{:?}", result);
}
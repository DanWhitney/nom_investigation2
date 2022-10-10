use nom::{IResult, bytes::complete::take};


fn take_n_parser<'a>(i: &'a str,number: usize) -> IResult<&'a str,&'a str> {
    take(number)(i)
}
fn main() {
    let result = take_n_parser("Hello, World!",4usize).unwrap();
    println!("{:?}", result);
}
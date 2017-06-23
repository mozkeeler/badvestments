pub mod badvestments;
pub mod ast;

fn main() {
    println!("You should buy some bitcoin");
    println!("{:?}", badvestments::parse_Rule("Badvice ::= Prefix \"quoted string with $*&^ stuff\" Vehicle"));
}

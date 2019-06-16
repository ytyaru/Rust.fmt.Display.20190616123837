/*
 * 構造体や列挙の独自表示を実装する。
 * CreatedAt: 2019-06-16
 * https://stackoverflow.com/questions/28024373/is-there-a-way-to-print-enum-values
 */
use std::fmt;
fn main() {
    // #[derive(Debug)]と{:?}{:#?}があればimplがなくとも名前を表示できる
//    println!("{:?}", Suit::Heart);
    // implすれば#[derive(Debug)], {:?}, {:#?}不要で指定の表示ができる
    println!("{}", Suit::Heart);
    println!("{}", Suit::Diamond);
    println!("{}", Suit::Spade);
    println!("{}", Suit::Club);
}
//#[derive(Debug)]
enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}
// error[E0277]: `Suit` doesn't implement `std::fmt::Display`
impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Suit::Heart => write!(f, "♥"),
            Suit::Diamond => write!(f, "♦"),
            Suit::Spade => write!(f, "♠"),
            Suit::Club => write!(f, "♣"),
        }
    }
}

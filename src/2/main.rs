/*
 * 構造体や列挙の独自表示を実装する。
 * CreatedAt: 2019-06-16
 * https://stackoverflow.com/questions/28024373/is-there-a-way-to-print-enum-values
 */
use std::fmt;
fn main() {
    println!("{}", Suit::Heart);
    println!("{}", Suit::Diamond);
    println!("{}", Suit::Spade);
    println!("{}", Suit::Club);
}
enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}
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

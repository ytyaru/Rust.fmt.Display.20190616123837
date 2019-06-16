/*
 * 構造体や列挙の独自表示を実装する。
 * CreatedAt: 2019-06-16
 * https://stackoverflow.com/questions/28024373/is-there-a-way-to-print-enum-values
 */
use std::fmt;
fn main() {
//    println!("{}", Suit::Heart); // error[E0277]: `Suit` doesn't implement `std::fmt::Display`    note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
    println!("{:?}", Suit::Heart); // error[E0277]: `Suit` doesn't implement `std::fmt::Display`    note: add `#[derive(Debug)]` or manually implement `std::fmt::Debug`
}
#[derive(Debug)]
enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}


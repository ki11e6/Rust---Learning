#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

fn main() {
    let dec = Deck { cards: vec![] };
    println!("Here is you deck:{:?}", dec);
}

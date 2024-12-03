#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

fn main() {
    let suits = ["hearts", "spades", "clubs", "diamonds"];
    let values = [
        "ace", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "jack",
        "queen", "king",
    ];

    let mut cards = vec![]; // add mute to make the binding mutable.

    for suit in suits {
        for value in values {
            let card = format!("{} of {}", value, suit); //format for joining.
            cards.push(card);
        }
    }

    let dec = Deck { cards: cards };
    println!("Here is you deck:{:#?}", dec);
}

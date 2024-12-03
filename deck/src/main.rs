#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
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
        return dec;
    }
}

fn main() {
    let dec = Deck::new();
    println!("Here is you deck:{:#?}", dec);
}

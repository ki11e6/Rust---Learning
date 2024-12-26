#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

//this is inherent implientation with impl
// impl is a keyword used to create an implementation block that add function to the struct.
//they have same name as struct that's why its called inherent implementation.
// impl is used to create methods and functions that are associated with the struct.
impl Deck {
    // return type inotation with return type Deck or Self as function is inside Deck and returning it to Dect parent.
    //this is associated function
    fn new() -> Self {
        //following are area the has fixed values
        let suits = ["hearts", "spades", "clubs", "diamonds"];
        let values = [
            "ace", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "jack",
            "queen", "king",
        ];

        //below is vector arrays that has dinamic array
        // let mut cards = vec::new(); /*you can also use this */
        let mut cards = vec![]; // add mute to make the binding mutable.

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit); //format for joining.
                cards.push(card);
            }
        }

        return Deck { cards };
    }
}

fn main() {
    let dec = Deck::new();
    println!("Here is you deck:{:#?}", dec);
}

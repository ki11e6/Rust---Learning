#[derive(Debug)]

enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    fn description(&self) -> String {
        // if let Media::Book { title, author } = self {
        //     format!("Book: {} {}", title, author)
        // } else if let Media::Movie { title, director } = self {
        //     format!("Movie: {} {}", title, director)
        // } else if let Media::Audiobook { title } = self {
        //     format!("Audiobook: {}", title)
        // } else {
        //     String::from("Media description!")
        // }
        // Pattern matching, using match instead of if let
        match self {
            Media::Book { title, author } => format!("Book: {} {}", title, author),
            Media::Movie { title, director } => format!("Movie: {} {}", title, director),
            Media::Audiobook { title } => format!("Audiobook: {}", title),
            Media::Podcast(id) => format!("Podcast episode: {}", id),
            Media::Placeholder => String::from("Placeholder!"),
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }
}

fn print_media(media: Media) {
    println!("{:#?}", media);
}
fn main() {
    let bad_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Unknown"),
    };

    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Famous Director"),
    };

    let cool_audiobook = Media::Audiobook {
        title: String::from("Cool Audiobook"),
    };

    let podcast = Media::Podcast(42);
    let placeholder = Media::Placeholder;
    // print_media(good_movie);
    // print_media(cool_audiobook);
    // print_media(bad_book);

    // println!("{:#?}", good_movie.description());
    // println!("{:#?}", cool_audiobook.description());
    // println!("{:#?}", bad_book.description());
    let mut catelog = Catalog::new();

    catelog.add(cool_audiobook);
    catelog.add(bad_book);
    catelog.add(good_movie);
    catelog.add(podcast);
    catelog.add(placeholder);

    match catelog.items.get(10) {
        Option::Some(value) => {
            println!("{}", value.description());
        }
        Option::None => {
            println!("No media found");
        }
    }
}

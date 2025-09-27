#[derive(Debug)]

enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
}

impl Media {
    fn description(&self) -> String {
        if let Media::Book { title, author } = self {
            format!("Book: {} {}", title, author)
        } else if let Media::Movie { title, director } = self {
            format!("Movie: {} {}", title, director)
        } else if let Media::Audiobook { title } = self {
            format!("Audiobook: {}", title)
        } else {
            String::from("Media description!")
        }
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
    // print_media(good_movie);
    // print_media(cool_audiobook);
    // print_media(bad_book);

    println!("{:#?}", good_movie.description())
}

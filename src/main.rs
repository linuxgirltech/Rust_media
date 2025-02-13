#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder
}

impl Media {
    fn description(&self) -> String {
        match self {
            Media::Book { title, author } => {
                format!("Book: {} by {}", title, author)
            },
            Media::Movie { title, director } => {
                format!("Movie: {} by {}", title, director)
            },
            Media::Audiobook { title } => {
                format!("Audiobook: called {}", title)
            },
            Media::Podcast(episode) => {
                format!("Podcast: episode {}", episode)
            },
            Media::Placeholder => {
                format!("Placeholder")
            }
        }
    }
}
#[derive(Debug)]
struct Catalog {
    items: Vec<Media>
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            Some(&self.items[index])
        } else {
            None
        }
        
    }
}

// fn print_media(media: Media) {
//     println!("{:#?}", media)
// }

fn main() {
    let audiobook = Media::Audiobook { 
        title: String::from("Project Hail Mary") 
    };
    let movie = Media::Movie { 
        title: String::from("The Matrix"), 
        director: String::from("The Wachowskis") 
    };
    let book = Media::Book { 
        title: String::from("The Lord of the Rings"), 
        author: String::from("J.R.R. Tolkein") 
    };
    let podcast = Media::Podcast(10);
    let placeholder = Media::Placeholder;


    let mut catalog = Catalog::new();

    // println!("{}", movie.description());
    // println!("{}", book.description());
    // println!("{}", audiobook.description())

//   print_media(audiobook);
//   print_media(movie);
//   print_media(book);

    catalog.add(audiobook);
    catalog.add(movie);
    catalog.add(book);
    catalog.add(podcast);
    catalog.add(placeholder);

    let item = catalog.get_by_index(0);
    
    // println!("{:#?}", item);
    match catalog.get_by_index(0) {
        Some(value) => {
            println!("Item: {:#?}", value);
        }
        None => {
            println!("No value here");
        }
    }
}
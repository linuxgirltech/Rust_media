mod content;

use content::media::Media;
use content::catalog::Catalog;

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

    catalog.add(audiobook);
    catalog.add(movie);
    catalog.add(book);
    catalog.add(podcast);
    catalog.add(placeholder);

    let item = catalog.get_by_index(40);
    let placeholder = Media::Placeholder;
    
    println!("{:#?}", item.unwrap_or(&placeholder));
}
#[derive(Debug)]
pub enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder
}

impl Media {
    pub fn description(&self) -> String {
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
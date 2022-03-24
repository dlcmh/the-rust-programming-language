pub trait Summary {
    // define a default implementation / behavior for the `summarize` method
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
// an empty `impl` block
impl Summary for NewsArticle {}

pub trait TweetSummary {
    // define a default implementation / behavior for the `summarize` method
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

    // a mandatory implementation
    fn summarize_author(&self) -> String;
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl TweetSummary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", article.summarize());
    // New article available! (Read more...)

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    // 1 new tweet: (Read more from @horse_ebooks...)
}

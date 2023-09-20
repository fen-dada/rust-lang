pub trait Summary {
    pub fn summarize(&self) -> String {
        String::from("(Read more...)") // default, need to be implemented with empty block
    }
}

pub struct tweet {
    pub headline: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub struct news {
    pub headline: String,
    pub location: String,
    pub content: String,
    pub author: String,
}

impl summary for news {
    pub fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

fn return_summarizable(switch: bool) -> impl summary {
    if switch {
        tweet {
            headline: String::from("q"),
            content: String::from("d"),
            reply: false,
            retweet: false,
        }
    }
}

impl<T: Display> ToString for T {
    // T impl Display, so we impl methods below to such T
    // --snip--
}

//as parameter
fn sum(item: &impl summary) {
    //pub fn notify(item: &(impl Display + summary))
    println!("{}", item.summarize());
}

//trait bound
fn sum_2<T: summary>(item: &T) {
    // pub fn notify<T: Display + summary>(item: &T)
    println!("{}", T.summarize());
}

fn some_function<T, U>(t: &T, u: &U)
where
    T: Display + summary,
    U: Debug + Clone,
{
}
fn main() {}

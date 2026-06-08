// shared behavior using traits

// assume we have a program which aggregates different types of text content
// in this case, a news article and a tweet
// now we want to have the ability to summarize the news article and the ability to summarize the
// tweet so we can post that in our text aggregation feed

use std::fmt::Debug;
use std::fmt::Display;

// in this case we can use a trait to define a shared behavior between the article and the tweet
// e.g. summarization
pub struct NewsArticle {
  pub author: String,
  pub headline: String,
  pub content: String,
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    format!("{}, by {}", self.headline, self.author)
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}

pub trait Summary {
  fn summarize(&self) -> String;

  // trait with default implementation
  fn default(&self) -> String {
    String::from("Read more...")
  }
}

// A function that takes an argument that expects
// the argument to have implemented the Summary trait!
pub fn notify(item: &impl Summary) {
  println!("Breaking news! {}", item.summarize());
}

// above function is a **Trait Bound** of the following syntax
// pub fn notify<T: Summary>(item: &T) {
//   println!("Breaking news! {}", item.summarize());
// }

// takes an argument that implements both summary and display
pub fn notifyItemsWMultipleTraits(item: &(impl Summary + Display)) {}

// pub fn notify<T: Summary>(item1: &T, items2: &T) {

// }

// takes an argument that implements both summary and display
// pub fn notifyItemsWMultipleTraits<T: Summary + Display>(item: T) {}

// where clause
fn some_function<T, U>(t: &T, u: &U) -> i32
where
  T: Display + Clone,
  U: Clone + Debug,
{
  return 0;
}

fn returns_summarizable() -> impl Summary {
  Tweet {
    username: String::from("@johndoe"),
    content: String::from("Hello world"),
    reply: false,
    retweet: false,
  }
}

// Illegal...
// Types have to be the same!
// fn returns_summarizable(switch: bool) -> impl Summary {
//   if switch {
//     NewsArticle {
//       author: String::from("John Doe"),
//       headline: String::from("The sky is falling!"),
//       content: String::from("The sky is not actually falling."),
//     }
//   } else {
//     Tweet {
//       username: String::from("@johndoe"),
//       content: String::from("Hello world"),
//       reply: false,
//       retweet: false,
//     }
//   }
// }

struct Pair<T> {
  x: T,
  y: T,
}

impl<T> Pair<T> {
  fn new(x: T, y: T) -> Self {
    Self { x, y }
  }
}

// use traits for conditional implementation
impl<T: Display + PartialOrd> Pair<T> {
  fn cmp_display(&self) {
    if self.x >= self.y {
      println!("The largest member is x = {}", self.x)
    } else {
      println!("The largest member is y = {}", self.y)
    }
  }
}

// blanket implementations
// implement a trait on a type that has another trait
pub trait MyCustomToString {
  fn to_custom_string(&self) -> String;
}

impl<T: Display> MyCustomToString for T {
  fn to_custom_string(&self) -> String {
    format!("New String: {}", self)
  }
}

fn main() {
  let tweet = Tweet {
    username: String::from("@johndoe"),
    content: String::from("Hello world"),
    reply: false,
    retweet: false,
  };

  let article = NewsArticle {
    author: String::from("John Doe"),
    headline: String::from("The sky is falling!"),
    content: String::from("The sky is not actually falling."),
  };

  println!("Tweet summary: {}", tweet.summarize());
  println!("Article summary: {}", article.summarize());
  println!("Default trait: {}", article.default());
  notify(&tweet);
}

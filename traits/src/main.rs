use traits::{Summary, Tweet, NewsArticle};
use core::fmt::Display;
use core::fmt::Debug;
fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}... Read more from {}", tweet.summarize(), tweet.summarize_author());

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
    
}
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
pub fn notifyButInAMoreVerboseWayToDeclareIt<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
pub fn notifyTwoFromTheSameType<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {}", item1.summarize());
}
pub fn notifyTwoWithoutNecessarilyHavingTheSameType(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {}", item1.summarize());
}
pub fn notifyWithMultipleTraitBound(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}
pub fn notifyWithMultipleTraitBoundButInAMoreVerboseWayToDeclareIt<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
//Múltiplas Traits e diferentes para cada tipo 
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
  4
}
//Uma maneira mais fácil de declarar múltiplas Traits e diferentes para cada tipo usando "where" keyword
fn some_function_two<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
  5
}
//Podemos definir que o tipo de retorno é um que Implementa uma trait em específico
fn returns_summarizable() -> impl Summary { //Isso só funciona se a função com certeza retorna apenas um tipo, se ela pode retornar um tipo ou outro irá gerar um erro
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
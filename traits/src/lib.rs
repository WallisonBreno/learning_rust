use std::fmt::Display;
// pub trait Summary {
//     fn summarize(&self) -> String; //funções que adotarem essa trait tem que construir seu próprio método summarize
// }
pub trait Summary {
    fn summarize_author(&self) -> String; //Terminando a assinatura da função com ';' não dá ela um padrão
    fn summarize(&self) -> String { //Dentro do corpo podemos ter uma implementação vazia
        format!("(Read more from {}...)", self.summarize_author()) //Podemos chamar outro método da trait dentro de outro
    }
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
//Para implementar uma trait a um tipo temos que usa essa sintaxe "impl <Trait> for <struct>"
//Podemos fazer essa relação de trait e struct se pelo menos 1 deles for local
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String { //Implementar uma trait com uma implementação padrão sobrepõe o padrão
        let resume = &self.content[0..10];
        format!("{}: {}", self.username, resume)
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
//Para receber a implementação padrão da trait nós implementamos ela sem dar seu corpo de função
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
// Usando "impl <traits> struct<generic>" permite que a função só seja definida caso o generic da struct seja do tipo que implementa as traits que estão dentro de '<>' após o 'impl'
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
//Podemos tambem implementar uma trait em um tipo genérico qualquer que implemente as traits dentro de '<>' 
//O exemplo abaixo é um usado no core do rust apenas para exemplificar
// impl<T: Display> ToString for T {
//     // --snip--
// }
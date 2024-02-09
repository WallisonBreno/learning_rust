// &i32        // uma referência
// &'a i32     // uma referência com um lifetime explicitado
// &'a mut i32 // uma referência mutável com um lifetime explicitado

use std::fmt::Display;

//Usando a sintaxe dessa forma definimos que a struct não pode ter um lifetime maior que a referência em part
struct ImportantExcerpt<'a> {
    part: &'a str,
}
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
fn main() {

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    //A referência passada para a struct não sai do escopo antes da struct, então funcionará
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    
    //Isso irá gerar um erro pois o result faz referência a algo que sai do escopo antes de result ser usado
    //Na declaração da nossa função explicitamos que o lifetime deve assumir o menor entre os lifetimes
    //Então mesmo que o lifetime da resposta seja válido o compilador não entende isso
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);
    
    //Existe tambem o lifetime static que define a duração da referência para o tempo de execução do programa inteiro
    let s: &'static str = "I have a static lifetime.";

}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
//Como sempre retornamos x podemos não explicitar o lifetime de 'y' 
fn longest_return_x_always<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

//Isso irá dar erro pois a resposta pode ser uma &str mas não tem uma lifetime relacionada a algum dos parâmetros
// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

//Aceita um tipo genérico e tem o lifetime que for menor entre x e y
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
use std::collections::HashMap;
use std::io;
fn main() {
    
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    scores.insert(String::from("Blue"), 20);
    scores.entry(String::from("Yellow")).or_insert(30);
    scores.entry(String::from("Yellow")).or_insert(40);

    let mut input = String::new();
    println!("Please input your phrase for us to count the words:");
    io::stdin().read_line(&mut input).expect("Did not enter a correct string");
    let text: &str = &input[..];
    count_words(text);

}

fn count_words(text: &str){
    let mut map = HashMap::new();

    for word in text.split_whitespace(){
        //É retornado uma referência mutável para nossa variável com o valor iniciado ou encontrado
        let count = map.entry(word).or_insert(0);
        //Adicionamos 1 ao valor
        *count += 1;
    }
    //Printamos na tela quantas vezes a palavra aparece na tela
    println!("{:?}", map);
}

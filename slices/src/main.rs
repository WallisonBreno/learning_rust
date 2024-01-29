fn main() {
    let s = String::from("hello world");
    //slices são operadores que retornam uma referência de porção de uma String
    let hello = &s[0..5];
    let world = &s[6..11];

    let mut s = String::from("hello world");
    let word = first_word(&s[..]); //Como o parâmetro da função é um &str devemos passar um slice da palavra completa
    //s.clear(); // erro! Pois já existe um ponteiro mutável refrenciando
    println!("the first word is: {}", word);

    let s = "Hello, world!"; //o tipo de s é &str, todas strings literais tem esse tipo e é por isso que string literais são imutáveis

    //Slices tambem podem ser usados em arrays.
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
    
}

fn first_word(s: &str) -> &str { // O tipo &str pode recebr String e &str
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

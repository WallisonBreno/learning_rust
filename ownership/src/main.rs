fn main() {

    let s1 = String::from("hello");
    let s2 = s1;
    //Isso irá gerar um erro, pois em rust apenas uma variável pode ser dona de ume valor, ao passar o valor de s1 para s2
    //Rust limpa s1 da memória movendo o valor de s1 para s2, o propósito disso é para eficiência de memória
    // println!("{}, world!", s1);
    println!("{}, world!", s2);

    let x = 4;
    let y = x;
    //Nesse caso não irá dar erro pois o tipo integer vai para a satck de execução, não a heap logo não há necessidade de otimzarção
    // na memória pois a stack já tem uma leitura bem rápida e intgers tem tambem um tamanho definido fazendo tudo ser mais rápido.
    println!("{x}");

    //Entretanto se a fução copy for usada, o comportamento do integer será igual na String
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    //O tipo string literal tambem é armazenada na stack por ter tamanho definido então tambem tem o mesmo comportamento do integer
    //Tipos com esse comportamento implementam a Trait "Copy"
    let text = "Olá, Mundo!";
    let text_copy = text;
    println!("text = {}, text_copy = {}", text, text_copy);

    let s = String::from("hello");  // s entra no escopo

    takes_ownership(s);             // s move seu valor para a função
                                    // Aqui já não é mais válido

    let x = 5;                      // x entra no escopo

    makes_copy(x);                  // x entra na função mas o tipo i32 implementa a Trait "Copy" entçao ele ainda é válido
    let s1 = gives_ownership(); // A funçao move seu valor para s1
    let s2 = String::from("hello"); 
    let s3 = takes_and_gives_back(s2) //Move o valor de s2 para função e a função move seu valor para s3

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1); //Moveu o valor de s1 para a função 
                                          //e a função retornou o valor e o tamanho do valor para s2 
    println!("The length of '{}' is {}.", s2, len);

}

fn takes_ownership(some_string: String) { // alguma String entra no escopo
    println!("{}", some_string);
} //Aqui alguma String sai do escopo

fn makes_copy(some_integer: i32) { // Algum integer entra no escopo
    println!("{}", some_integer);
} // Aqui um integer sai do escopo

fn gives_ownership() -> String {     

    let some_string = String::from("yours"); // String entra no escopo

    some_string         // retorna String
}
fn takes_and_gives_back(a_string: String) -> String { // a_string entra no escopo

    a_string  // a_string é retornada e movida para algum outro lugar

}
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() retorna o tamanho da String

    (s, length) //retornando uma tupla com a String e seu tamanho
}
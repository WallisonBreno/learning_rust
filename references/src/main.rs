fn main() {

    let s1 = String::from("hello");
    //Usando a keyword '&' o valor passado na função se torna uma referência, logo o valor não é dado a função, ela recebe apenas seu endereço de memória como um ponteiro.
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len); // A variável continua s1 sendo válida pois ela não perdeu seu valor. 

    //Para mudar um valor por referência a variável deve ser mutável e passada para a função especificando isso, coma keyword '&mut'
    //Mas referências mutáveis tem uma restrição: Só pode haver uma referência mutável não usada para cada valor, não pode ser criado 2 referência mutáveis para o mesmo valor antes que ao menos uma seja usada antes.
    let mut s = String::from("hello");
    change(&mut s);

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // usando escopo 2 referências podem ser criadas antes do uso da outra portanto que estejam em diferentes escopos

    let r2 = &mut s;

    let mut s = String::from("hello");

    let r1 = &s; // sem problema
    let r2 = &s; // sem problema
    //let r3 = &mut s; // PROBLEMA

    println!("{}, {}", r1, r2);

    let mut s = String::from("hello");

    let r1 = &s; // sem problemas
    let r2 = &s; //sem problemas
    println!("{} and {}", r1, r2);
    // ambas foram usadas

    let r3 = &mut s; // logo, aqui não tem problema
    println!("{}", r3);

}

fn calculate_length(s: &String) -> usize { //O parâmetro deve ser uma referência.
    s.len()
}
fn change(some_string: &mut String) { //Para mudar um valor por referência a variável deve ser mutável e o parâmetro deve ser uma referência mutável
    some_string.push_str(", world");
}
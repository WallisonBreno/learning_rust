fn main() {

    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    //Isso irá gerar um erro pois rust não converte integers para booleanos
    // let number = 3;
    // if number {
    //     println!("number was three");
    // }

    //Já esse irá rodar pois a expressão de comparação retorna um booleano
    let number = 3;
    if number != 0 {
        println!("number was something other than zero");
    }

    let number = 6;

    //else if
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    //if e else para declarar variáveis, para isso o bloco deve retornar uma expressão que não termina com ';'
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    //Isso irá gerar um erro pois o tipo de retorno do else é diferente do if
    // let condition = true;

    // let number = if condition { 5 } else { "six" };

    // println!("The value of number is: {number}");

    //O 'loop' gera um loop infinito mas que pode ser interrompido
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // loops podem ter labels que podem ser usadas para interrompe-las especificamente
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    //while
    let mut number = 3;
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!!!");

    //Iterar por um array usando while
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    //Usando 'for' para iterar por um array
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    //Contador usando 'for' passando o range(number..number) dos números que devem ser visitados e revertendo o range unsado 'rev'
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

}

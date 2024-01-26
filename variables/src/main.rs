use std::io;
fn main() {
    // Isoo vai gerar um erro pois está tentando atribuir um novo valor a uma variável imutável
    // let x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    //Isso vai dar certo Pois x foi declarada como mutável pelo uso da keyword 'mut'
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    //Constante :) (Com o tipo unsigned integer 32bits)
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // Shadowing, Declarar uma variável de mesmo identificador de outra, deixando a outra para trás
    // Uso de escopo, após o fim do escopo o shadwoing anterior termina
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    //Com shadowing uma nova variável é criado, logo pode ser de tipos diferentes
    let spaces = "   ";
    let spaces = spaces.len();
    //Usando a keyword 'mut' a troca de tipos não é possível e resultará em erro.
    // let mut spaces = "   ";
    // spaces = spaces.len();

    //Tipos

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

        // adição
        let sum = 5 + 10;

        // subtração
        let difference = 95.5 - 4.3;
    
        // multiplicação
        let product = 4 * 30;
    
        // divisão
        let quotient = 56.7 / 32.2;
        let truncated = -5 / 3; // Resulta em -1
    
        // resto
        let remainder = 43 % 5;

        let t = true; // tipagem implicita

        let f: bool = false; // tipagem explicita

        let z: char = 'ℤ'; 
        let heart_eyed_cat = '😻';

        //tupla
        let tup = (500, 6.4, 1);

        //desestruturação de tupla
        let (x, y, z) = tup;
        println!("The value of y is: {y}");

        let x: (i32, f64, u8) = (500, 6.4, 1);
        //desestruturação de tupla usando índices
        let five_hundred = x.0;

        let six_point_four = x.1;

        let one = x.2;

        //arrays
        let a = [1, 2, 3, 4, 5];

        //array com tipo;tamanho definido
        let a: [i32; 5] = [1, 2, 3, 4, 5];
        
        // Inicializando um array que todos os itens terão valor 3
        //Coloque o valor default e termine com um ';' para isso
        //Depois do ';' pode ser definido um tamanho
        let a = [3; 5]; // isso é o mesmo que = "let a = [3, 3, 3, 3, 3];"
        
        //Acessando itens do array
        let a = [1, 2, 3, 4, 5];

        let first = a[0];
        let second = a[1];

        //Tratando entradas inválidas de chave
        //Se for inputado um numero maior que o do tamanho do array dará erro
        let a = [1, 2, 3, 4, 5];

        println!("Please enter an array index.");
    
        let mut index = String::new();
    
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");
    
        let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");
    
        let element = a[index];
    
        println!("The value of the element at index {index} is: {element}");

}

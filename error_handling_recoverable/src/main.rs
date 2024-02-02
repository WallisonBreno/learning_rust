use std::fs::File;
use std::io::{self, Read,ErrorKind};
fn main() {
    
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let greeting_file_result = File::open("hello.txt");
    //O atributo kind retorna o tipo de erro que pode ser tratado por um match tambem
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            //Verifica o tipo de erro que aconteceu no terminal
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            //Resposta default caso dê um erro
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
    //Maneira mais limpa verificar error usando closure e o método unwrap_or_else
    //unwrap_or_else verifica se existe um valor do tipo Some e retorna se tiver, se não retornar executa a closure
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    let greeting_file = File::open("hello.txt").unwrap(); //Usando o unwrap irá retornar o resultado da Option, se for um Err irá causar um erro
    let greeting_file = File::open("hello.txt")
    .expect("hello.txt should be included in this project"); //usando expect a mesma coisa é feita mas se for um Err ele trata a mensagem de erro 

}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e), //usando return para "propagar o erro" esse termo é usado para retornar o erro para a função que chamou essa função
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e), //Aqui não foi necessário usar o return pois é a última linha e ela automáticamente retorna, propagando o erro
    }
}
fn read_username_from_file_shorter() -> Result<String, io::Error> {
    //o operador "?" faz o match dos valores, se for Ok o resultado da expressão é Ok, senão propagará um Err convertendo para o tipo que foi definido no tipo de retorno da função
    //Tudo isso propagando da mesma maneira que o outro método
    //Converter o tipo de erro pode ser útil caso você queira fazer um tipo custom de Err
    //Esse operador funciona da mesma maneira do match mas com a conversão do tipo e eliminando várias linhas de código, entretanto só pode ser usado em funções com o tipo de retorno Result ou Option(Não pode misturar os 2)
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?; 
    Ok(username)
}
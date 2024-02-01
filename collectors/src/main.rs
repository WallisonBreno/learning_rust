
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
fn main() {

    //Vectors só guardam 1 tipo, então se você não inicializar com algum valor, o tipo deve ser explicitado
    let v: Vec<i32> = Vec::new();

    let v = vec![1,2,3];

    let mut v = Vec::new();
    //Na função push v é passado como referência mutável, então não podemos criar outra referência após o push
    v.push(5);
    v.push(6);
    v.push(7);

    //Acessando valores no Vector
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");
    //Usando o método get recebemos uma option com uma referência ao tipo do vector
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    //Iterando sob referência imutável de cada item do array
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    //Iterando sob referência mutável de cada item do array
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        //Para mudar o valor dessa referência mutável devemos usar o operador "*"
        *i += 50;

    }

    //Guardando valores do mesmo enum, Mas com diferentes tipos
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    match &row[1] {
        SpreadsheetCell::Int(i) => println!("{}",i),
        _ => println!("Is not a integer")
    }
}

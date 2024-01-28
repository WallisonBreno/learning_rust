fn main() {

    another_function(5);
    print_labeled_measurement(5, 'h');

    //Statements em rust não retornam um valor, mas expressões sim, e expressões não terminam em ';', então para retornar o valor de x para y
    //Escrevemos uma expressão "x + 1".
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");

}
fn another_function(x:i32) {
    println!("The value of x is: {x}");
}
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
//Em rust com a keyword '->' é definido o tipo de retorno e o retorno é automaticamente dado pela última expressão
//A keyword "retutn" tambem pode ser usada.
fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1
}
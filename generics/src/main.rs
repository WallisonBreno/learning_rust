//Generics é um tipo genérico que pode aceitar um ou mais tipos concretos
//Dentro de <> definimos o generic e se ele vai seguir alguma trait(comportamento) usando a keyword ':'
//Nesse generic definimos que ele tem que seguir a trait PartialOrd, significando que ele tem um comportamento de ordenação e logo pode ser comparado
//Se tirarmos essa trait teremos um erro pois dentro da função há uma comparação.
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}
//Definir método em uma struct com Generics, Podia ser outro nome para o Generic mas por convenção usamos o mesmo
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
//Podemos declarar métodos específicos para tipos específicos dos Generics
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
struct PointTwoGenerics<T, U> {
    x: T,
    y: U,
}
impl<T, U> PointTwoGenerics<T, U> {
    //Método que recebe a instância que chama o método (PointTwoGenerics<T, U>) e um tipo PointTwoGenerics com Generics diferentes(PointTwoGenerics<T2, U2>)
    //E retorna um generic da instância que foi chamado o método e outra generic da struct que foi passada como argumento
    fn mixup<T2, U2>(self, other: PointTwoGenerics<T2, U2>) -> PointTwoGenerics<T, U2> {
        PointTwoGenerics {
            x: self.x,
            y: other.y,
        }
    }
}

//Generics em Enums
enum Example<T> {
    Generic(T),
    NonGeneric,
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    //Passando uma lista de inteiros, se tivesse mais de 1 tipo daria erro pois o generic por vez só aceita 1 
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    //Passando uma lista de chars, se tivesse mais de 1 tipo daria erro pois o generic por vez só aceita 1  
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    //let wont_work = Point { x: 5, y: 4.0 };

    //generic por vez só aceita 1, então com 2 generics ele consegue suportar mais de 1 tipo 
    let both_integer = PointTwoGenerics { x: 5, y: 10 };
    let both_float = PointTwoGenerics { x: 1.0, y: 4.0 };
    let integer_and_float = PointTwoGenerics { x: 5, y: 4.0 };

    //Criamos uma Struct com 2 tipos, Outra com 2 tipo e misturamos o x de uma com o y de outra
    //Fizemos isso pois já é passado um tipo PointTwoGenerics<T, U> para o método e definimos mais uma struct com generics diferentes
    //Dessa forma aceitando até 4 valores diferentes.
    let p1 = PointTwoGenerics { x: 5, y: 10.4 };
    let p2 = PointTwoGenerics { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

}
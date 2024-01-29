#[derive(Debug)] //Comando para ser possível printar essa struct
struct Rectangle {
    width: u32,
    height: u32,
}
//Usando a keyword 'impl' podemos abrir um bloco para criar métodos
impl Rectangle {
    fn area(&self) -> u32 { //&self é um shorthand para "rectangle: &Rectangle", é uma referência imutável para a instância que chama a função
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self { //Métodos que não tem '&self' como seu primeiro parâmetro são chamadas de funções associativas, elas não são chamadas por uma instância mas pela própria struct
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = (30, 50);
    println!("The area of the rectangle is {} square pixels.",areaUsingTuples(rect1));

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:?}", rect1); //Usando keyword ':?' podemos printar uma struct
    println!("The area of the rectangle is {} square pixels.",areaUsingStruct(&rect1));
    //Usando a macro dbg podemos printar tambem e ainda mostrar a linha onde foi printada. Para usar tem qeu ter o "#[derive(Debug)]" acima da struct
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);

    //Usando methods
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3); // Usando a keyword '::' podemos chamar funções associativas, sendo chamadas na struct em si

}

fn areaUsingTuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
fn areaUsingStruct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
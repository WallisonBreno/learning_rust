struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
//Structs de tuplas
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//unit-like Struct
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    let user2 = build_user(String::from("example@gmail.com"), String::from("example"));

    let user3 = User {
        active: user1.active,
        username: String::from("nome"),
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count, //Usando campos de user 1 para criar o 3
    };

    let user4 = User {
        email: String::from("another@example.com"),
        ..user1 // Com o operador '..' copiamos os outros atributos de outra instância para a nova instância, ele deve vir por último para pegar apenas os atributos restantes
        //Entretanto essa ação está sob efeito das regras de ownership logo user1 não pode mais ser usado, pois seus valores pertencem a user4
        // Se tivessemos criado um nome e um email para user4 e copiado apenas os outros campos de user1 ele ainda poderia ser usado, pois o tipo bool e u64 usam a trait "Copy"
    };
    //Note que as duas variáveis tem valores diferentes pois tem tipos diferentes
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;

}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

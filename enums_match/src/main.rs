#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x { //matchs precisam de todos os casos possíveis, se tirarmos o traatamento de None irá dar um erro
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}

fn remove_fancy_hat() {}

fn move_player(num_spaces: u8) {}

fn reroll() {}

fn main() {
    let coin = Coin::Quarter(UsState::Alaska);
    println!("{}",value_in_cents(&coin));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll { //Colocando um valor como 'other' por último torna ele default.
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
        // _ => reroll(), Isso tambem funciona
        // _ => (), Isso tambem funciona

    }

    //O if let é uma forma menos verbosa de escrever um match, pegando apenas a condição após o "if let"]
    //Aqui se a variável for do tipo Some() ele realiza uma ação
    let config_max = Some(3u8);
    if let Some(max) = config_max { //passando a variável config_max para a verificação
        println!("The maximum is configured to be {}", max);
    }

    //Pode tambem ser usado um else caso não satisfaça a condição
    let mut count = 0;
    if let Coin::Quarter(state) = coin { //passando a variável coin para a verificação
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }


}

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn main() {
    println!("Value of {:#?} in pence is {}", Coin::Coin1p, value_in_pence(Coin::Coin1p));
    for coin in Coin::iter() {
        println!("{:?}", coin);
    }

    let salary = Some(Coin::Coin2pound);
    if let Some(dosh) = salary  {
        println!("Payday!, {:?}", dosh);
    } else {
        println!("No payday");
    }
}

#[derive(Debug, EnumIter)]
enum Coin {
    Coin1p,
    Coin2p,
    Coin5p,
    Coin10p,
    Coin20p,
    Coin50p,
    Coin1pound,
    Coin2pound,
}

fn value_in_pence(coin: Coin) -> i32 {
    match coin {
        Coin::Coin1p => 1,
        Coin::Coin2p => 2,
        Coin::Coin5p => 5,
        Coin::Coin10p => 10,
        Coin::Coin20p => 20,
        Coin::Coin50p => 50,
        Coin::Coin1pound => 100,
        Coin::Coin2pound => 200,
    }
}
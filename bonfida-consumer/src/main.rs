use bonfida;
use bonfida::api::*;
use bonfida::market::Market;

fn main() {
    println!("Hello, world!");

    //bonfida::testing();

    let market: Market = Bonfida::new();
    // let result = general.get_all_pairs();

    // match result {
    //     Ok(result) => println!("{:?}", result.data),
    //     Err(er) => println!("{:?}", er),
    // }
}

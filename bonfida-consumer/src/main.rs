use bonfida;
use bonfida::api::*;
use bonfida::market::Market;

fn main() {
    println!("Hello, world!");

    //bonfida::testing();

    let market: Market = Bonfida::new();
    // let result = market.get_all_pairs();

    // match result {
    //     Ok(result) => println!("{:?}", result.data),
    //     Err(er) => println!("{:?}", er),
    // }

    // let result = market.get_all_24hr_trades();
    // match result {
    //     Ok(result) => println!("{:?}", result.data),
    //     Err(er) => println!("{:?}", er),
    // }

    // let result = market.get_trades_for_market(String::from("FIDARAY"));
    // match result {
    //     Ok(result) => println!("{:?}", result.data),
    //     Err(er) => println!("{:?}", er),
    // }

    // let result = market.get_order_books_for_market(String::from("FIDARAY"));
    // match result {
    //     Ok(result) => println!("{:?}", result.data),
    //     Err(er) => println!("{:?}", er),
    // }

    // let result = market.get_volumes_for_market(String::from("FIDARAY"));
    // match result {
    //     Ok(result) => println!("{:?}", result.data),
    //     Err(er) => println!("{:?}", er),
    // }

    let result = market.get_trades_for_market_address(String::from(
        "9wH4Krv8Vim3op3JAu5NGZQdGxU8HLGAHZh3K77CemxC",
    ));
    match result {
        Ok(result) => println!("{:?}", result.data),
        Err(er) => println!("{:?}", er),
    }
}

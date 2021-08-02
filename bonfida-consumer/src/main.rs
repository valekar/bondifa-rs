use bonfida;
use bonfida::api::*;
use bonfida::general::General;

fn main() {
    println!("Hello, world!");

    //bonfida::testing();

    let general: General = Bonfida::new();
    let result = general.get_all_pairs();

    match result {
        Ok(result) => println!("{:?}", result.data),
        Err(er) => println!("{:?}", er),
    }
}

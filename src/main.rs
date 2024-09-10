mod coin;
mod produto;
mod bank;
mod blockchain;

use produto::Produto;
use coin::Coin;
use bank::calc_portifolio;
use bank::create_account;
use blockchain::create_block;




fn main() {
    println!("----------------------------------------------------");
    println!("portifolio bancario");
    calc_portifolio();

    let mut coin = Coin::new(1);
    println!("{}", coin.get_value());

    coin.set_value(2);
    println!("{}", coin.get_value());

    println!("----------------------------------------------------");
    println!("produtos");
    let mut produto = Produto::new(String::from("Caique"), 10.0, 1);
    println!("{:#?}", produto.get_product());

    produto.new_name(String::from("Tenis"));
    produto.new_price(20.0);
    produto.new_quantity(10);
    println!("{:#?}", produto.get_product());

    println!("----------------------------------------------------");
    println!("blockchain");
    create_block();
    println!("----------------------------------------------------");
    println!("bank currency");
    let data_bank = create_account();
    
}

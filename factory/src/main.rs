mod pizza_store;
mod new_york_pizza;
mod chicago_pizza;
mod orenburg_pizza;

mod dumb_pizza_store;

use pizza_store::*;
use new_york_pizza::*;
use chicago_pizza::*;
use orenburg_pizza::*;
use dumb_pizza_store::DumbPizzaStore;

fn main() {
    /*
    let factory = Box::new( OrenburgPizzaFactory::new() );
    let store = PizzaStore::new(factory);

    let pizza = store.order_pizza("Аля-Соль-Илецк");
    println!("Pizza: {:#?}", pizza);
    */

    
    let store = DumbPizzaStore::new("Orenburg");
    let pizza = store.order_pizza("Аля-Соль-Илецк");
    println!("Pizza: {:#?}", pizza);
    
}

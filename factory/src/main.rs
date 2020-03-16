mod pizza_store;
mod new_york_pizza;
mod chicago_pizza;
mod orenburg_pizza;

use pizza_store::*;
use new_york_pizza::*;
use chicago_pizza::*;
use orenburg_pizza::*;


fn main() {
    let factory = Box::new( OrenburgPizzaFactory::new() );
    let store = PizzaStore::new(factory);

    let pizza = store.order_pizza("Аля-Соль-Илецк");
    println!("Pizza: {:#?}", pizza);
}

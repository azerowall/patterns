pub mod ingredient;
pub mod ingredient_factory;
pub mod pizza;
pub mod pizza_factory;

use pizza_factory::PizzaFactory;
use pizza::Pizza;

pub struct PizzaStore {
    pizza_factory: Box<dyn PizzaFactory>,
}

impl PizzaStore {
    pub fn new(pizza_factory: Box<dyn PizzaFactory>) -> PizzaStore {
        PizzaStore {
            pizza_factory
        }
    }

    pub fn order_pizza(&self, name: &str) -> Pizza {
        let pizza = self.pizza_factory.create_pizza(name);
        pizza.bake();
        pizza.cut();
        pizza.pack_in_box();
        pizza
    }
}
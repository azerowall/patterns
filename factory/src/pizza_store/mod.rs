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
    pub fn new(pizza_factory: Box<dyn PizzaFactory>) -> Self {
        Self {
            pizza_factory
        }
    }

    pub fn order_pizza(&self, name: &str) -> Box<dyn Pizza> {
        let mut pizza = self.pizza_factory.create_pizza(name);
        pizza.prepare();
        pizza.bake();
        pizza.cut();
        pizza.pack_in_box();
        pizza
    }
}
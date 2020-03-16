pub mod ingredient_factory;
pub mod cheese_pizza;
pub mod greek_pizza;
pub mod pepperoni_pizza;
pub mod sol_iletsk_pizza;

use crate::pizza_store::{
    pizza::Pizza,
    pizza_factory::PizzaFactory,
};
use ingredient_factory::OrenburgIngredientFactory;
use cheese_pizza::OrenburgCheesePizza;
use greek_pizza::OrenburgGreekPizza;
use pepperoni_pizza::OrenburgPepperoniPizza;
use sol_iletsk_pizza::OrenburgSolIletskPizza;


pub struct OrenburgPizzaFactory;

impl PizzaFactory for OrenburgPizzaFactory {
    fn create_pizza(&self, pizza_type: &str) -> Box<dyn Pizza> {
        let name = format!("Orenburg {} pizza", pizza_type);
        
        match pizza_type {
            "cheese" => Box::new( OrenburgCheesePizza::new(&name, OrenburgIngredientFactory{}) ),
            "greek" => Box::new( OrenburgGreekPizza::new(&name, OrenburgIngredientFactory{}) ),
            "pepperoni" => Box::new( OrenburgPepperoniPizza::new(&name, OrenburgIngredientFactory{}) ),
            "Аля-Соль-Илецк" => Box::new( OrenburgSolIletskPizza::new(&name, OrenburgIngredientFactory{}) ),
            _ => panic!("OMAGAD UNKNOWN PIZZA")
        }
    }
}

impl OrenburgPizzaFactory {
    pub fn new() -> Self {
        Self {}
    }
}
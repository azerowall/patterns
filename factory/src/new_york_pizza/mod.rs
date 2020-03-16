pub mod ingredient_factory;
pub mod cheese_pizza;
pub mod greek_pizza;
pub mod pepperoni_pizza;

use crate::pizza_store::{
    pizza::Pizza,
    pizza_factory::PizzaFactory,
};
use ingredient_factory::NYIngredientFactory;
use cheese_pizza::NYCheesePizza;
use greek_pizza::NYGreekPizza;
use pepperoni_pizza::NYPepperoniPizza;


pub struct NYPizzaFactory;

impl PizzaFactory for NYPizzaFactory {
    fn create_pizza(&self, pizza_type: &str) -> Box<dyn Pizza> {
        let name = format!("New York {} pizza", pizza_type);
        
        match pizza_type {
            "cheese" => Box::new( NYCheesePizza::new(&name, NYIngredientFactory{}) ),
            "greek" => Box::new( NYGreekPizza::new(&name, NYIngredientFactory{}) ),
            "pepperoni" => Box::new( NYPepperoniPizza::new(&name, NYIngredientFactory{}) ),
            _ => panic!("OMAGAD UNKNOWN PIZZA")
        }
    }
}

impl NYPizzaFactory {
    pub fn new() -> Self {
        Self {}
    }
}
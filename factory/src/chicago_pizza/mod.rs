pub mod ingredient_factory;
pub mod cheese_pizza;
pub mod greek_pizza;
pub mod pepperoni_pizza;

use crate::pizza_store::{
    pizza::Pizza,
    pizza_factory::PizzaFactory,
};
use ingredient_factory::ChicagoIngredientFactory;
use cheese_pizza::ChicagoCheesePizza;
use greek_pizza::ChicagoGreekPizza;
use pepperoni_pizza::ChicagoPepperoniPizza;


pub struct ChicagoPizzaFactory;

impl PizzaFactory for ChicagoPizzaFactory {
    fn create_pizza(&self, pizza_type: &str) -> Box<dyn Pizza> {
        let name = format!("Chicago {} pizza", pizza_type);
        
        match pizza_type {
            "cheese" => Box::new( ChicagoCheesePizza::new(&name, ChicagoIngredientFactory{}) ),
            "greek" => Box::new( ChicagoGreekPizza::new(&name, ChicagoIngredientFactory{}) ),
            "pepperoni" => Box::new( ChicagoPepperoniPizza::new(&name, ChicagoIngredientFactory{}) ),
            _ => panic!("OMAGAD UNKNOWN PIZZA")
        }
    }
}

impl ChicagoPizzaFactory {
    pub fn new() -> Self {
        Self {}
    }
}
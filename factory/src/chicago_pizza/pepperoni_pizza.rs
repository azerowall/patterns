use crate::pizza_store::{
    ingredient::*,
    ingredient_factory::IngredientFactory,
    pizza::Pizza,
};
use super::ingredient_factory::ChicagoIngredientFactory;

#[derive(Debug)]
pub struct ChicagoPepperoniPizza {
    name: String,
    dough: Option<Dough>,
    sauce: Option<Sauce>,
    veggies: Vec<Veggie>,
    pepperoni: Option<Pepperoni>,
    factory: ChicagoIngredientFactory,
}

impl ChicagoPepperoniPizza {
    pub fn new(name: &str, factory: ChicagoIngredientFactory) -> Self {
        Self {
            name: String::from(name),
            dough: None,
            sauce: None,
            veggies: Vec::new(),
            pepperoni: None,
            factory,
        }
    }
}

impl Pizza for ChicagoPepperoniPizza {
    fn prepare(&mut self) {
        self.dough = Some(self.factory.create_dough());
        self.sauce = Some(self.factory.create_sauce());
        self.veggies = self.factory.create_veggies();
        self.pepperoni = Some(self.factory.create_pepperoni());
    }
}
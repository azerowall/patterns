use crate::pizza_store::{
    ingredient::*,
    ingredient_factory::IngredientFactory,
    pizza::Pizza,
};
use super::ingredient_factory::NYIngredientFactory;

#[derive(Debug)]
pub struct NYPepperoniPizza {
    pub name: String,
    pub dough: Option<Dough>,
    pub sauce: Option<Sauce>,
    pub veggies: Vec<Veggie>,
    pub pepperoni: Option<Pepperoni>,
    pub factory: NYIngredientFactory,
}

impl NYPepperoniPizza {
    pub fn new(name: &str, factory: NYIngredientFactory) -> Self {
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

impl Pizza for NYPepperoniPizza {
    fn prepare(&mut self) {
        self.dough = Some(self.factory.create_dough());
        self.sauce = Some(self.factory.create_sauce());
        self.veggies = self.factory.create_veggies();
        self.pepperoni = Some(self.factory.create_pepperoni());
    }
}
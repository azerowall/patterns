use crate::pizza_store::{
    ingredient::*,
    ingredient_factory::IngredientFactory,
    pizza::Pizza,
};
use super::ingredient_factory::ChicagoIngredientFactory;

#[derive(Debug)]
pub struct ChicagoCheesePizza {
    name: String,
    dough: Option<Dough>,
    sauce: Option<Sauce>,
    cheese: Option<Cheese>,
    factory: ChicagoIngredientFactory,
}

impl ChicagoCheesePizza {
    pub fn new(name: &str, factory: ChicagoIngredientFactory) -> Self {
        Self {
            name: String::from(name),
            dough: None,
            sauce: None,
            cheese: None,
            factory,
        }
    }
}

impl Pizza for ChicagoCheesePizza {
    fn prepare(&mut self) {
        self.dough = Some(self.factory.create_dough());
        self.sauce = Some(self.factory.create_sauce());
        self.cheese = Some(self.factory.create_cheese());
    }
}
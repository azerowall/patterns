use crate::pizza_store::{
    ingredient::*,
    ingredient_factory::IngredientFactory,
    pizza::Pizza,
};
use super::ingredient_factory::NYIngredientFactory;

#[derive(Debug)]
pub struct NYCheesePizza {
    pub name: String,
    pub dough: Option<Dough>,
    pub sauce: Option<Sauce>,
    pub cheese: Option<Cheese>,
    pub factory: NYIngredientFactory,
}

impl NYCheesePizza {
    pub fn new(name: &str, factory: NYIngredientFactory) -> Self {
        Self {
            name: String::from(name),
            dough: None,
            sauce: None,
            cheese: None,
            factory,
        }
    }
}

impl Pizza for NYCheesePizza {
    fn prepare(&mut self) {
        self.dough = Some(self.factory.create_dough());
        self.sauce = Some(self.factory.create_sauce());
        self.cheese = Some(self.factory.create_cheese());
    }
}
use crate::pizza_store::{
    ingredient::*,
    ingredient_factory::IngredientFactory,
    pizza::Pizza,
};
use super::ingredient_factory::OrenburgIngredientFactory;

#[derive(Debug)]
pub struct OrenburgCheesePizza {
    pub name: String,
    pub dough: Option<Dough>,
    pub sauce: Option<Sauce>,
    pub cheese: Option<Cheese>,
    pub factory: OrenburgIngredientFactory,
}

impl OrenburgCheesePizza {
    pub fn new(name: &str, factory: OrenburgIngredientFactory) -> Self {
        Self {
            name: String::from(name),
            dough: None,
            sauce: None,
            cheese: None,
            factory,
        }
    }
}

impl Pizza for OrenburgCheesePizza {
    fn prepare(&mut self) {
        self.dough = Some(self.factory.create_dough());
        self.sauce = Some(self.factory.create_sauce());
        self.cheese = Some(self.factory.create_cheese());
    }
}
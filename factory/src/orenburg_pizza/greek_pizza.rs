use crate::pizza_store::{
    ingredient::*,
    ingredient_factory::IngredientFactory,
    pizza::Pizza,
};
use super::ingredient_factory::OrenburgIngredientFactory;


#[derive(Debug)]
pub struct OrenburgGreekPizza {
    pub name: String,
    pub dough: Option<Dough>,
    pub sauce: Option<Sauce>,
    pub veggies: Vec<Veggie>,
    pub clam: Option<Clam>,
    pub factory: OrenburgIngredientFactory,
}

impl OrenburgGreekPizza {
    pub fn new(name: &str, factory: OrenburgIngredientFactory) -> Self {
        Self {
            name: String::from(name),
            dough: None,
            sauce: None,
            veggies: Vec::new(),
            clam: None,
            factory,
        }
    }
}

impl Pizza for OrenburgGreekPizza {
    fn prepare(&mut self) {
        self.dough = Some(self.factory.create_dough());
        self.sauce = Some(self.factory.create_sauce());
        self.veggies = self.factory.create_veggies();
        self.clam = Some(self.factory.create_clam());
    }
}
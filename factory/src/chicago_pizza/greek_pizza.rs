use crate::pizza_store::{
    ingredient::*,
    ingredient_factory::IngredientFactory,
    pizza::Pizza,
};
use super::ingredient_factory::ChicagoIngredientFactory;


#[derive(Debug)]
pub struct ChicagoGreekPizza {
    name: String,
    dough: Option<Dough>,
    sauce: Option<Sauce>,
    veggies: Vec<Veggie>,
    clam: Option<Clam>,
    factory: ChicagoIngredientFactory,
}

impl ChicagoGreekPizza {
    pub fn new(name: &str, factory: ChicagoIngredientFactory) -> Self {
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

impl Pizza for ChicagoGreekPizza {
    fn prepare(&mut self) {
        self.dough = Some(self.factory.create_dough());
        self.sauce = Some(self.factory.create_sauce());
        self.veggies = self.factory.create_veggies();
        self.clam = Some(self.factory.create_clam());
    }
}
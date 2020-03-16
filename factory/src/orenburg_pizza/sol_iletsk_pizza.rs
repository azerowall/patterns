use crate::pizza_store::{
    ingredient::*,
    ingredient_factory::IngredientFactory,
    pizza::Pizza,
};
use super::ingredient_factory::OrenburgIngredientFactory;


#[derive(Debug)]
pub struct OrenburgSolIletskPizza {
    pub name: String,
    pub dough: Option<Dough>,
    pub sauce: Option<Sauce>,
    pub veggies: Vec<Veggie>,
    pub clam: Option<Clam>,
    pub specific_ingredient: Option<SpecificIngredient>,
    pub factory: OrenburgIngredientFactory,
}

impl OrenburgSolIletskPizza {
    pub fn new(name: &str, factory: OrenburgIngredientFactory) -> Self {
        Self {
            name: String::from(name),
            dough: None,
            sauce: None,
            veggies: Vec::new(),
            clam: None,
            specific_ingredient: None,
            factory,
        }
    }
}

impl Pizza for OrenburgSolIletskPizza {
    fn prepare(&mut self) {
        self.dough = Some(self.factory.create_dough());
        self.sauce = Some(self.factory.create_sauce());
        self.veggies = self.factory.create_veggies();
        self.clam = Some(self.factory.create_clam());
        self.specific_ingredient = Some(self.factory.create_specific_ingredient());
    }
}
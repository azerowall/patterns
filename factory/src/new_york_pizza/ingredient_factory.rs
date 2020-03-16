use crate::pizza_store::{
    ingredient::*,
    ingredient_factory::IngredientFactory,
};

#[derive(Debug)]
pub struct NYIngredientFactory;

impl IngredientFactory for NYIngredientFactory {
    fn create_dough(&self) -> Dough {
        Dough::ThinCrustDough
    }
    fn create_sauce(&self) -> Sauce {
        Sauce::MarinaraSauce
    }
    fn create_veggies(&self) -> Vec<Veggie> {
        vec![Veggie::Garlic, Veggie::Onion, Veggie::Mushroom, Veggie::RedPepper]
    }
    fn create_cheese(&self) -> Cheese {
        Cheese::ReggianoCheese
    }
    fn create_pepperoni(&self) -> Pepperoni {
        Pepperoni::SlicedPepperoni
    }
    fn create_clam(&self) -> Clam {
        Clam::FreshClam
    }
}
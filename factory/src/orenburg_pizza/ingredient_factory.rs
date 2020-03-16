use crate::pizza_store::{
    ingredient::*,
    ingredient_factory::IngredientFactory,
};

#[derive(Debug)]
pub struct OrenburgIngredientFactory;

impl OrenburgIngredientFactory {
    pub fn create_specific_ingredient(&self) -> SpecificIngredient {
        SpecificIngredient::SaltedWatermelon
    }
}

impl IngredientFactory for OrenburgIngredientFactory {
    fn create_dough(&self) -> Dough {
        Dough::ThickCrustDough
    }
    fn create_sauce(&self) -> Sauce {
        Sauce::PlumTomatoSauce
    }
    fn create_veggies(&self) -> Vec<Veggie> {
        vec![Veggie::Onion, Veggie::Mushroom, Veggie::RedPepper]
    }
    fn create_cheese(&self) -> Cheese {
        Cheese::MozarellaCheese
    }
    fn create_pepperoni(&self) -> Pepperoni {
        Pepperoni::NotSlicedPepperoni
    }
    fn create_clam(&self) -> Clam {
        Clam::FrozenClam
    }
}
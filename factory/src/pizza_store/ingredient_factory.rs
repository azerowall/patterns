use super::ingredient::*;


pub trait IngredientFactory {
    fn create_dough(&self) -> Dough;
    fn create_sauce(&self) -> Sauce;
    fn create_veggies(&self) -> Vec<Veggie>;
    fn create_cheese(&self) -> Cheese;
    fn create_pepperoni(&self) -> Pepperoni;
    fn create_clam(&self) -> Clam;
    fn create_specific_additive(&self) -> SpecificAdditive;
}
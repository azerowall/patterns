use crate::pizza_store::{
    ingredient::*,
    ingredient_factory::IngredientFactory,
    pizza::Pizza,
    pizza_factory::PizzaFactory,
};


pub struct ChicagoIngredientFactory;

impl IngredientFactory for ChicagoIngredientFactory {
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
    fn create_specific_additive(&self) -> SpecificAdditive {
        SpecificAdditive::StrawberryJam
    }
}


pub struct ChicagoPizzaFactory {
    ingredient_factory: Box<dyn IngredientFactory>,
}

impl PizzaFactory for ChicagoPizzaFactory {
    fn create_pizza(&self, pizza_type: &str) -> Pizza {

        let factory = &self.ingredient_factory;
        let pizza_name = format!("Chicago {} pizza", pizza_type);
        match pizza_type {
            "cheese" => {
                let mut pizza = Pizza::new(&pizza_name);
                pizza
                    .dough(factory.create_dough())
                    .sauce(factory.create_sauce())
                    .veggies(factory.create_veggies())
                    .cheese(factory.create_cheese());
                pizza
            },
            "greek" => {
                let mut pizza = Pizza::new(&pizza_name);
                pizza
                    .dough(factory.create_dough())
                    .sauce(factory.create_sauce())
                    .veggies(factory.create_veggies())
                    .clam(factory.create_clam());
                pizza
            },
            "pepperoni" => {
                let mut pizza = Pizza::new(&pizza_name);
                pizza
                    .dough(factory.create_dough())
                    .sauce(factory.create_sauce())
                    .pepperoni(factory.create_pepperoni())
                    .clam(factory.create_clam());
                pizza
            }
            _ => panic!("OMAGAD UNKNOWN PIZZA")
        }
    }
}

impl ChicagoPizzaFactory {
    pub fn new() -> Self {
        ChicagoPizzaFactory {
            ingredient_factory: Box::new( ChicagoIngredientFactory{} )
        }
    }
}
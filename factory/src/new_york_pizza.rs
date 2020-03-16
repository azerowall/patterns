use crate::pizza_store::{
    ingredient::*,
    ingredient_factory::IngredientFactory,
    pizza::Pizza,
    pizza_factory::PizzaFactory,
};


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
    fn create_specific_additive(&self) -> SpecificAdditive {
        SpecificAdditive::Pineapple
    }
}


pub struct NYPizzaFactory {
    ingredient_factory: Box<dyn IngredientFactory>,
}

impl PizzaFactory for NYPizzaFactory {
    fn create_pizza(&self, pizza_type: &str) -> Pizza {

        let factory = &self.ingredient_factory;
        let name = format!("New York {} pizza", pizza_type);
        match pizza_type {
            "cheese" => {
                let mut pizza = Pizza::new(&name);
                pizza
                    .dough(factory.create_dough())
                    .sauce(factory.create_sauce())
                    .cheese(factory.create_cheese());
                pizza
            },
            "greek" => {
                let mut pizza = Pizza::new(&name);
                pizza
                    .dough(factory.create_dough())
                    .sauce(factory.create_sauce())
                    .veggies(factory.create_veggies())
                    .clam(factory.create_clam());
                pizza
            },
            "pepperoni" => {
                let mut pizza = Pizza::new(&name);
                pizza
                    .dough(factory.create_dough())
                    .sauce(factory.create_sauce())
                    .pepperoni(factory.create_pepperoni());
                pizza
            }
            _ => panic!("OMAGAD UNKNOWN PIZZA")
        }
    }
}

impl NYPizzaFactory {
    pub fn new() -> NYPizzaFactory {
        NYPizzaFactory {
            ingredient_factory: Box::new( NYIngredientFactory{} )
        }
    }
}
use crate::pizza_store::{
    ingredient::*,
    ingredient_factory::IngredientFactory,
    pizza::Pizza,
    pizza_factory::PizzaFactory,
};


pub struct OrenburgIngredientFactory;

impl IngredientFactory for OrenburgIngredientFactory {
    fn create_dough(&self) -> Dough {
        Dough::ThickCrustDough
    }
    fn create_sauce(&self) -> Sauce {
        Sauce::PlumTomatoSauce
    }
    fn create_veggies(&self) -> Vec<Veggie> {
        vec![Veggie::Onion, Veggie::Garlic]
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
        SpecificAdditive::SaltedWatermelon
    }
}


pub struct OrenburgPizzaFactory {
    ingredient_factory: Box<dyn IngredientFactory>,
}

impl PizzaFactory for OrenburgPizzaFactory {
    fn create_pizza(&self, pizza_type: &str) -> Pizza {

        let factory = &self.ingredient_factory;
        let name = format!("Orenburg {} pizza", pizza_type);
        match pizza_type {
            "cheese" => {
                let mut pizza = Pizza::new(&name);
                pizza
                    .dough(factory.create_dough())
                    .sauce(factory.create_sauce())
                    .veggies(factory.create_veggies())
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
                    .pepperoni(factory.create_pepperoni())
                    .clam(factory.create_clam());
                pizza
            },
            "Аля-Соль-Илецк" => {
                let mut pizza = Pizza::new(&name);
                pizza
                    .dough(factory.create_dough())
                    .sauce(factory.create_sauce())
                    .clam(factory.create_clam())
                    .specific_additive(factory.create_specific_additive());
                pizza
            },
            _ => panic!("OMAGAD UNKNOWN PIZZA")
        }
    }
}

impl OrenburgPizzaFactory {
    pub fn new() -> Self {
        OrenburgPizzaFactory {
            ingredient_factory: Box::new( OrenburgIngredientFactory{} )
        }
    }
}
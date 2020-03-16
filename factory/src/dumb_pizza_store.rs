use crate::pizza_store::{
    ingredient::*,
    ingredient_factory::IngredientFactory,
    pizza::Pizza,
};
use crate::new_york_pizza::{
    ingredient_factory::NYIngredientFactory,
    cheese_pizza::NYCheesePizza,
    greek_pizza::NYGreekPizza,
    pepperoni_pizza::NYPepperoniPizza,
};

use crate::orenburg_pizza::{
    ingredient_factory::OrenburgIngredientFactory,
    cheese_pizza::OrenburgCheesePizza,
    greek_pizza::OrenburgGreekPizza,
    pepperoni_pizza::OrenburgPepperoniPizza,
    sol_iletsk_pizza::OrenburgSolIletskPizza,
};


pub struct DumbPizzaStore {
    city: String
}

impl DumbPizzaStore {
    pub fn new(city: &str) -> Self {
        Self { city: String::from(city) }
    }

    pub fn order_pizza(&self, pizza_type: &str) -> Box<dyn Pizza> {
        let name = format!("{} {} pizza", self.city, pizza_type);

        let mut pizza: Box<dyn Pizza> = match &self.city as &str {
            "New York" => match pizza_type {
                "cheese" => Box::new( NYCheesePizza {
                    name,
                    factory: NYIngredientFactory {}, // не используется
                    dough: Some(Dough::ThinCrustDough),
                    sauce: Some(Sauce::MarinaraSauce),
                    cheese: Some(Cheese::ReggianoCheese),
                }),
                "greek" => Box::new( NYGreekPizza {
                    name,
                    factory: NYIngredientFactory {},
                    dough: Some(Dough::ThinCrustDough),
                    sauce: Some(Sauce::MarinaraSauce),
                    veggies: vec![Veggie::Garlic, Veggie::Onion, Veggie::Mushroom, Veggie::RedPepper],
                    clam: Some(Clam::FreshClam),
                }),
                "pepperoni" => Box::new( NYPepperoniPizza {
                    name,
                    factory: NYIngredientFactory {},
                    dough: Some(Dough::ThinCrustDough),
                    sauce: Some(Sauce::MarinaraSauce),
                    veggies: vec![Veggie::Garlic, Veggie::Onion, Veggie::Mushroom, Veggie::RedPepper],
                    pepperoni: Some(Pepperoni::SlicedPepperoni),
                }),
                _ => panic!("OMAGAD UNKNOWN PIZZA")
            },
            "Orenburg" => match pizza_type {
                "cheese" => Box::new (OrenburgCheesePizza {
                    name,
                    factory: OrenburgIngredientFactory{},
                    dough: Some(Dough::ThickCrustDough),
                    sauce: Some(Sauce::PlumTomatoSauce),
                    cheese: Some(Cheese::MozarellaCheese),
                }),
                "greek" => Box::new (OrenburgGreekPizza {
                    name,
                    factory: OrenburgIngredientFactory{},
                    dough: Some(Dough::ThickCrustDough),
                    sauce: Some(Sauce::PlumTomatoSauce),
                    veggies: vec![Veggie::Onion, Veggie::Mushroom, Veggie::RedPepper],
                    clam: Some(Clam::FrozenClam),
                }),
                "pepperoni" => Box::new (OrenburgPepperoniPizza {
                    name,
                    factory: OrenburgIngredientFactory{},
                    dough: Some(Dough::ThickCrustDough),
                    sauce: Some(Sauce::PlumTomatoSauce),
                    veggies: vec![Veggie::Onion, Veggie::Mushroom, Veggie::RedPepper],
                    pepperoni: Some(Pepperoni::NotSlicedPepperoni),
                }),
                "Аля-Соль-Илецк" => Box::new (OrenburgSolIletskPizza {
                    name,
                    factory: OrenburgIngredientFactory{},
                    dough: Some(Dough::ThickCrustDough),
                    sauce: Some(Sauce::PlumTomatoSauce),
                    veggies: vec![Veggie::Onion, Veggie::Mushroom, Veggie::RedPepper],
                    clam: Some(Clam::FrozenClam),
                    specific_ingredient: Some(SpecificIngredient::SaltedWatermelon),
                }),
                _ => panic!("OMAGAD UNKNOWN PIZZA")
            }

            _ => panic!("OMAGAD UNKNOWN CITY")
        };

        pizza.bake();
        pizza.cut();
        pizza.pack_in_box();

        pizza
    }
}
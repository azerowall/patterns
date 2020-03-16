use super::ingredient::*;


#[derive(Debug)]
pub struct Pizza {
    name: String,
    dough: Option<Dough>,
    sauce: Option<Sauce>,
    veggies: Vec<Veggie>,
    cheese: Option<Cheese>,
    pepperoni: Option<Pepperoni>,
    clam: Option<Clam>,
    specific_additive: Option<SpecificAdditive>,
}

impl Pizza {
    pub fn new(name: &str) -> Self {
        Pizza {
            name: String::from(name),
            dough: None,
            sauce: None,
            veggies: Vec::new(),
            cheese: None,
            pepperoni: None,
            clam: None,
            specific_additive: None,
        }
    }
    
    pub fn dough(&mut self, dough: Dough) -> &mut Self {
        self.dough = Some(dough);
        self
    }
    pub fn sauce(&mut self, sauce: Sauce) -> &mut Self {
        self.sauce = Some(sauce);
        self
    }
    pub fn veggies(&mut self, veggies: Vec<Veggie>) -> &mut Self {
        self.veggies = veggies;
        self
    }
    pub fn cheese(&mut self, cheese: Cheese) -> &mut Self {
        self.cheese = Some(cheese);
        self
    }
    pub fn pepperoni(&mut self, pepperoni: Pepperoni) -> &mut Self {
        self.pepperoni = Some(pepperoni);
        self
    }
    pub fn clam(&mut self, clam: Clam) -> &mut Self {
        self.clam = Some(clam);
        self
    }
    pub fn specific_additive(&mut self, specific_additive: SpecificAdditive) -> &mut Self {
        self.specific_additive = Some(specific_additive);
        self
    }

    pub fn bake(&self) {
        println!("Bake for 25 minutes at 250");
    }

    pub fn cut(&self) {
        println!("Cutting into slices");
    }

    pub fn pack_in_box(&self) {
        println!("Place pizza in box");
    }
}

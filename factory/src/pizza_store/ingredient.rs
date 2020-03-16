#[derive(Debug)]
pub enum Dough {
    ThickCrustDough,
    ThinCrustDough,
}

#[derive(Debug)]
pub enum Sauce {
    PlumTomatoSauce,
    MarinaraSauce,
}

#[derive(Debug)]
pub enum Cheese {
    MozarellaCheese,
    ReggianoCheese,
}

#[derive(Debug)]
pub enum Clam {
    FrozenClam,
    FreshClam,
}

#[derive(Debug)]
pub enum Pepperoni {
    SlicedPepperoni,
    NotSlicedPepperoni,
}

#[derive(Debug)]
pub enum Veggie {
    Garlic,
    Onion,
    Mushroom,
    RedPepper,
}

#[derive(Debug)]
pub enum SpecificIngredient {
    SaltedWatermelon,
    Honey,
    Pineapple,
    StrawberryJam,
}
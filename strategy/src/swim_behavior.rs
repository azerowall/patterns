
pub trait SwimBehavior {
    fn swim(&self);
}

pub struct Swim {}
impl SwimBehavior for Swim {
    fn swim(&self) {
        println!("Swim!");
    }
}

pub struct NoSwim {}
impl SwimBehavior for NoSwim {
    fn swim(&self) {
        println!("I can't swim (");
    }
}

pub trait QuackBehavior {
    fn quack(&self);
}


pub struct Quack {}
impl QuackBehavior for Quack {
    fn quack(&self) {
        println!("Quack!");
    }
}

pub struct NoQuack {}
impl QuackBehavior for NoQuack {
    fn quack(&self) {
        println!("*siiileeence*");
    }
}
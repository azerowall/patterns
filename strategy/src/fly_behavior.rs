


pub trait FlyBehavior {
    fn fly(&self);
}

pub struct Fly {}
impl FlyBehavior for Fly {
    fn fly(&self) {
        println!("Fly!");
    }
}

pub struct NoFly {}
impl FlyBehavior for NoFly {
    fn fly(&self) {
        println!("I can't fly (");
    }
}
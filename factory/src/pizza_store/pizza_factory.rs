use super::pizza::Pizza;

pub trait PizzaFactory {
    fn create_pizza(&self, name: &str) -> Pizza;
}
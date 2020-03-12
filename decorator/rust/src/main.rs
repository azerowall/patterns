extern crate counter;

use counter::Counter;

//use std::collections::HashMap;


trait Beverage {
	fn get_description(&self) -> String {
		let components = self.get_components();
		return components.join(", ");
		let counts : Counter<_> = components.iter().collect();
		let counts = counts.into_map();
		let mut desc = String::new();
		for (key, value) in &counts {
			desc.push_str(key);
			desc.push_str(" = ");
			desc.push_str(&value.to_string());
			desc.push_str(", ");
		}
		desc
	}
	fn get_components(&self) -> Vec<&str> {
		vec![ self.get_own_description() ]
	}
	fn get_own_description(&self) -> &str;
	fn cost(&self) -> f64;
}

struct DarkRoast {}

impl Beverage for DarkRoast {
	fn get_own_description(&self) -> &str { "DarkRoast" }
	fn cost(&self) -> f64 { 0.84 }
}


trait BeverageDecorator : Beverage {
	fn new(beverage: Box<dyn Beverage>) -> Box<dyn Beverage>;
	fn get_inner_beverage(&self) -> &Box<dyn Beverage>;
}




struct Milk {
	beverage: Box<dyn Beverage>
}
impl BeverageDecorator for Milk {
	fn new(beverage: Box<dyn Beverage>) -> Box<dyn Beverage> {
		Box::new(Milk { beverage })
	}
	fn get_inner_beverage(&self) -> &Box<dyn Beverage> { &self.beverage }
}
impl Beverage for Milk {
	fn get_own_description(&self) -> &str { "Milk" }
	fn cost(&self) -> f64 { self.beverage.cost() + 0.15 }
}

/*
struct Mocha {
	beverage: Box<dyn Beverage>
}

impl BeverageDecorator for Mocha {
	fn new(beverage: Box<dyn Beverage>) -> Box<dyn Beverage> {
		Mocha { beverage }
	}
}
*/


fn main() {
	//let beverage = Box::new(DarkRoast{});
	//let beverage = Milk::new(beverage);
	//let beverage = Milk::new(beverage);
	
	//println!("Ваша овсянка, сэр: {}", beverage.get_description());

	let s = String::from("Hello world");
	let slice = s[..];
	println!("{}", slice);
}

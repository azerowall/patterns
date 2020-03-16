mod quack_behavior;
mod fly_behavior;
mod swim_behavior;

use quack_behavior::*;
use fly_behavior::*;
use swim_behavior::*;



struct Duck {
    quack_behavior: Box<dyn QuackBehavior>,
    fly_behavior: Box<dyn FlyBehavior>,
    swim_behavior: Box<dyn SwimBehavior>,
}

impl Duck {
    fn set_quack_behavior(&mut self, behavior: Box<dyn QuackBehavior>) {
        self.quack_behavior = behavior;
    }
    fn quack(&self) {
        self.quack_behavior.quack();
    }

    fn set_swim_behavior(&mut self, behavior: Box<dyn SwimBehavior>) {
        self.swim_behavior = behavior;
    }
    fn swim(&self) {
        self.swim_behavior.swim();
    }

    fn set_fly_behavior(&mut self, behavior: Box<dyn FlyBehavior>) {
        self.fly_behavior = behavior;
    }
    fn fly(&self) {
        self.fly_behavior.fly();
    }
}

impl Duck {
    fn new_mallard_duck() -> Duck {
        Duck {
            quack_behavior: Box::new(Quack{}),
            swim_behavior: Box::new(Swim{}),
            fly_behavior: Box::new(Fly{}),
        }
    }
}

impl Duck {
    fn new_wood_duck() -> Duck {
        Duck {
            quack_behavior: Box::new(NoQuack{}),
            swim_behavior: Box::new(Swim{}),
            fly_behavior: Box::new(NoFly{}),
        }
    }
}

impl Duck {
    fn new_roast_duck() -> Duck {
        Duck {
            quack_behavior: Box::new(NoQuack{}),
            swim_behavior: Box::new(NoSwim{}),
            fly_behavior: Box::new(NoFly{}),
        }
    }
}

struct DuckQuailCall {
    quack_behavior: Box<dyn QuackBehavior>,
}

impl DuckQuailCall {
    fn quack(&self) {
        self.quack_behavior.quack();
    }
}

impl DuckQuailCall {
    fn new_mallard_duck_quail_call() -> DuckQuailCall {
        DuckQuailCall {
            quack_behavior: Box::new(Quack{})
        }
    }
}



fn main() {
    let duck = Duck::new_wood_duck();

    duck.quack();
    duck.fly();
    duck.swim();

    let quail_call = DuckQuailCall::new_mallard_duck_quail_call();
    quail_call.quack();
}
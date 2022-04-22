pub trait AbstractionInterface {
    fn operation(&self) -> String;
}

pub trait ConcreteAbstraction {
    fn operation(&self) -> String {
        "ConcreteAbstraction".to_string()
    }
}

pub struct Abstraction {
    implementation: Box::<dyn Implementation>,
}

impl AbstractionInterface for Abstraction {
    fn operation(&self) -> String {
        self.implementation.impl_operation()
    }
}

pub trait Implementation {
    fn impl_operation(&self) -> String;
}

pub struct ConcreteImplementationA {
    content: String,
}

impl Implementation for ConcreteImplementationA {
    fn impl_operation(&self) -> String {
        self.content.clone()
    }
}

pub struct ConcreteImplementationB {}

impl Implementation for ConcreteImplementationB {
    fn impl_operation(&self) -> String {
        "ConcreteImplementationB".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bridge() {
	// switching the implementation field of the `Abstraction` during
	// runtime to select a concrete implementation
        let abstraction: Box<dyn AbstractionInterface> = Box::new(Abstraction {
            implementation: Box::new(ConcreteImplementationB {}),
        });
	let res = abstraction.operation();
	let exp = "ConcreteImplementationB".to_string();
	assert_eq!(exp, res);

	// other abstraction
	let abstraction = Box::new(Abstraction {
	    implementation: Box::new(ConcreteImplementationA { content: "ConcreteA".into() }),
	});

	// same implementation usage
	let res = abstraction.operation();
	let exp = "ConcreteA".to_string();
	assert_eq!(exp, res);
    }
}

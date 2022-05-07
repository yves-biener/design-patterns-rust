pub trait Component {
    fn operation(&self) -> String;
}

pub struct Decorator {
    component: Box<dyn Component>,
}

impl Component for Decorator {
    fn operation(&self) -> String {
        self.component.operation() + " Decorated"
    }
}

pub struct ConcreteComponent {
    name: String,
}

impl Component for ConcreteComponent {
    fn operation(&self) -> String {
        self.name.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decorator() {
	// basic component
	let component: Box<dyn Component> = Box::new(ConcreteComponent {
	    name: "ConcreteComponent".to_string(),
	});
	assert_eq!("ConcreteComponent", component.operation());

	// decorated component
	let component: Box<dyn Component> = Box::new(Decorator {
	    component,
	});
	assert_eq!("ConcreteComponent Decorated", component.operation());
    }
}

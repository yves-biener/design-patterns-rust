pub trait AbstractFactory {
    fn create_product_a(&self) -> Box<dyn ProductA>;
    fn create_product_b(&self) -> Box<dyn ProductB>;
}

pub trait ProductA {
    fn functionality(&self) -> String;
}

pub trait ProductB {
    fn functionality(&self) -> String;
}

struct ProductA1;

impl ProductA for ProductA1 {
    fn functionality(&self) -> String {
        "ProductA1".to_string()
    }
}

struct ProductA2;

impl ProductA for ProductA2 {
    fn functionality(&self) -> String {
        "ProductA2".to_string()
    }
}

struct ProductB1;

impl ProductB for ProductB1 {
    fn functionality(&self) -> String {
        "ProductB1".to_string()
    }
}

struct ProductB2;

impl ProductB for ProductB2 {
    fn functionality(&self) -> String {
        "ProductB2".to_string()
    }
}

struct ConcreteFactoryA;

impl AbstractFactory for ConcreteFactoryA {
    fn create_product_a(&self) -> Box<dyn ProductA> {
        Box::new(ProductA1)
    }

    fn create_product_b(&self) -> Box<dyn ProductB> {
        Box::new(ProductB1)
    }
}

struct ConcreteFactoryB;

impl AbstractFactory for ConcreteFactoryB {
    fn create_product_a(&self) -> Box<dyn ProductA> {
        Box::new(ProductA2)
    }

    fn create_product_b(&self) -> Box<dyn ProductB> {
        Box::new(ProductB2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory() {
	// create a factory
	let factory: Box<dyn AbstractFactory> = Box::new(ConcreteFactoryA);
	let product_a: Box<dyn ProductA> = factory.create_product_a();
	let product_b: Box<dyn ProductB> = factory.create_product_b();
	let result_a = product_a.functionality();
	let result_b = product_b.functionality();
	assert_eq!(String::from("ProductA1"), result_a);
	assert_eq!(String::from("ProductB1"), result_b);
	// use a different factory
	let factory: Box<dyn AbstractFactory> = Box::new(ConcreteFactoryB);
	let product_a: Box<dyn ProductA> = factory.create_product_a();
	let product_b: Box<dyn ProductB> = factory.create_product_b();
	let result_a = product_a.functionality();
	let result_b = product_b.functionality();
	assert_eq!(String::from("ProductA2"), result_a);
	assert_eq!(String::from("ProductB2"), result_b);
    }
}

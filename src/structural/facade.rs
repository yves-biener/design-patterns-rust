pub struct Facade {}

impl Facade {
    #[allow(dead_code)]
    pub fn simple_interface() -> Vec<String> {
	let mut complex_a = ComplexA::complex_interface();
	complex_a.append(&mut ComplexB::complex_interface());
	complex_a
    }
}

// unit struct
pub struct ComplexA {}

impl ComplexA {
    #[allow(dead_code)]
    pub fn complex_interface() -> Vec<String> {
	vec!["A".to_string(), "B".to_string(), "C".to_string()]
    }
}

pub struct ComplexB {}

impl ComplexB {
    #[allow(dead_code)]
    pub fn complex_interface() -> Vec<String> {
	vec!["C".to_string(), "D".to_string(), "E".to_string()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_facade() {
	let res = Facade::simple_interface();
	let exp = vec!["A".to_string(),
		       "B".to_string(),
		       "C".to_string(),
		       "C".to_string(),
		       "D".to_string(),
		       "E".to_string(),
	];
	assert_eq!(exp, res);
    }
}

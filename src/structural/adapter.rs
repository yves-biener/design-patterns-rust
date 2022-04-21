pub trait Target {
    fn operation(&self, x: i32) -> Option<i32>;
}

struct Adaptee {}

impl Adaptee {
    fn specific_operation(&self, x: i32) -> i32 {
        x * x
    }
}

pub struct Adapter {
    adaptee: Adaptee,
}

impl Target for Adapter {
    fn operation(&self, x: i32) -> Option<i32> {
        match x {
            _ if x == 0 => {
                println!("x == 0 is an invalid input");
                None
            }
            _ => {
                println!("Log: succesfully called operation on Adapter");
                Some(self.adaptee.specific_operation(x))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adapter() {
        let target: Box<dyn Target> = Box::new(Adapter {
            adaptee: Adaptee {},
        });
	let exp = 5 * 5;
	let res = target.operation(5);
	let res = match res {
	    Some(r) => r,
	    None => panic!("Adapter gave None instead of {exp}"),
	};
	assert_eq!(exp, res);
    }

    #[test]
    fn test_adapter_none_case() {
        let target: Box<dyn Target> = Box::new(Adapter {
            adaptee: Adaptee {},
        });
	let exp = None;
	let res = target.operation(0);
	assert_eq!(exp, res);
    }
}

pub trait Subject {
    fn print(&self, message: String) -> String;
}

// unit struct
pub struct Concrete;

impl Subject for Concrete {
    fn print(&self, message: String) -> String {
	format!("Concrete: {message}")
    }
}

pub struct Proxy {
    concrete: Concrete
}

impl Proxy {
    fn pre(&self) {
	println!("Proxy: pre");
    }

    fn post(&self) {
	println!("Proxy: post");
    }

    fn cond(&self, message: String) -> String {
	format!("Proxy: additional functionality -> {message}")
    }
}

impl Subject for Proxy {
    fn print(&self, message: String) -> String {
	self.pre();
	let mut message = self.concrete.print(message);
	// the additonal functionality will be called in certain conditions
	if message == "Concrete: 42" {
	    message = self.cond(message);
	}
	self.post();
	message
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concrete_subject() {
	let subject: Box<dyn Subject> = Box::new(Concrete);
	let res = subject.print("Test".into());
	let exp = format!("Concrete: Test");
	assert_eq!(exp, res);
    }

    #[test]
    fn test_proxy_subject() {
	let subject: Box<dyn Subject> = Box::new(Proxy {concrete: Concrete});
	let res = subject.print("Test".into());
	let exp = format!("Concrete: Test");
	assert_eq!(exp, res);
    }

    #[test]
    fn test_proxy_cond() {
	let subject: Box<dyn Subject> = Box::new(Proxy {concrete: Concrete});
	let res = subject.print("42".into());
	let exp = format!("Proxy: additional functionality -> Concrete: 42");
	assert_eq!(exp, res);
    }
}

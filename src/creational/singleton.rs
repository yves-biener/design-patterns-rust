#[allow(dead_code)]
struct Singleton {
    count: u32,
}

#[allow(dead_code)]
static mut SINGLETON: Singleton = Singleton {
    count: 0,
};

impl Singleton {
    #[allow(dead_code)]
    pub fn print(&mut self, message: &str) {
	println!("{message}");
	self.count += 1;
    }

    #[allow(dead_code)]
    pub fn count(&self) -> u32 {
	self.count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singleton() {
	// this pattern should rather not be used in Rust
	unsafe {
	    SINGLETON.print("Test");
	    let exp = 1;
	    let res = SINGLETON.count();
	    assert_eq!(exp, res);
	}
	// different scope but still same 'global' state
	unsafe {
	    SINGLETON.print("Another Test");
	    let exp = 2;
	    let res = SINGLETON.count();
	    assert_eq!(exp, res);
	}
    }
}

pub trait Command {
    fn get_name(&self) -> String;
    fn get_description(&self) -> String;
    fn execute(&self) -> String;
}

pub struct Concrete {
    name: String,
    description: String,
}

impl Command for Concrete {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_description(&self) -> String {
        self.description.clone()
    }

    fn execute(&self) -> String {
        format!("Simulating execution of task {} ...", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command() {
        let command: Box<dyn Command> = Box::new(Concrete {
            name: "Command Name".into(),
            description: "Command description".into(),
        });
        let res = "Simulating execution of task Command Name ...".to_string();
        let exp = command.execute();
        assert_eq!(exp, res);
    }
}

struct Leaf {
    name: String,
}

struct Composite<'a> {
    children: Vec<Box<&'a dyn Component>>,
}

trait Component {
    fn operation(&self) -> String;
}

impl Component for Leaf {
    fn operation(&self) -> String {
        // println!("{}", self.name);
        self.name.clone()
    }
}

impl<'a> Component for Composite<'a> {
    fn operation(&self) -> String {
        let mut res = String::new();
        for child in &self.children {
            let child_res = child.operation();
            if res.len() > 0 {
                res = format!("{}-{}", res, child_res);
            } else {
                res = child_res;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_composite() {
        // NOTE: the creation of the composite is still quite bad and seems to be wrong for the
        // correct way rust would like me to do it
        let leaf1 = Leaf {
            name: String::from("First Leaf"),
        };
        let first: Box<&dyn Component> = Box::new(&leaf1);
        let leaf2 = Leaf {
            name: String::from("Second Leaf"),
        };
        let second: Box<&dyn Component> = Box::new(&leaf2);
        let leaf3 = Leaf {
            name: String::from("Third Leaf"),
        };
        let third: Box<&dyn Component> = Box::new(&leaf3);
        let tree = Composite {
            children: vec![first, second],
        };
        let first_tree: Box<&dyn Component> = Box::new(&tree);
        let root = Composite {
            children: vec![first_tree, third],
        };

        let expected = String::from("First Leaf-Second Leaf-Third Leaf");
        let res = root.operation();

        assert_eq!(expected, res);
    }
}

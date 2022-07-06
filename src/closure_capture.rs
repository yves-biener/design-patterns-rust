///! taken from:
///! https://smallcultfollowing.com/babysteps/blog/2018/04/24/rust-pattern-precise-closure-capture-clauses/
///! A closure in rust captures all local variables used in the scope of the closure. This may lead
///! to some undesired behavior (as shown in the example). With this pattern you can create closure
///! captures (similar to the ones you can define in c++) to capture only the variables which you
///! require.
use std::collections::HashMap;

struct Context {
    input: HashMap<String, u32>,
    output: Vec<u32>,
}

// problem example:
impl Context {
    fn process_problem(&mut self, values: &[String]) {
        self.output.extend(
            values
                .iter()
                .map(|v| self.input.get(v).cloned().unwrap_or(0)), // this will cause an error, as output is mutable borrowed and input will be
                                                                   // borrowed, can not at the same time as output is already mutable borrowed!
        );
    }
}

// solution:
impl Context {
    fn process_solved(&mut self, values: &[String]) {
        self.output.extend(values.iter().map({
            // define "closure" variables
            // this may also copy / clone the values if required
            let input = &self.input;
            // use only these variables in the closure itself
            move |v| input.get(v).cloned().unwrap_or(0)
        }));
    }

    // if a bounded "closure" variable is required in multiple closures the following could also be
    // used instead
    fn process_multi_closure(&mut self, values: &[String]) {
        // define 'global' "closure" variables
        let input = &self.input;
        // use these variables (even in multiple different contextes)
        self.output
            .extend(values.iter().map(|v| input.get(v).cloned().unwrap_or(0)));
    }

    // also possible, but not desired:
    fn process_alternative(&mut self, values: &[String]) {
        for v in values {
            self.output.push(self.input.get(v).cloned().unwrap_or(0));
        }
    }
}

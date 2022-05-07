#[derive(Debug, Eq, PartialEq, Clone)]
#[allow(dead_code)]
pub enum State {
    Starting,
    Running,
    ShuttingDown,
}

#[allow(dead_code)]
pub struct Memento {
    state: State,
}

impl Memento {
    #[allow(dead_code)]
    pub fn new(state: State) -> Memento {
	Memento {state}
    }

    #[allow(dead_code)]
    pub fn get_state(&self) -> State {
	self.state.clone()
    }
}

#[allow(dead_code)]
struct Originator {
    pub state: State,
}

impl Originator {
    #[allow(dead_code)]
    pub fn restore(&mut self, memento: Memento) {
	self.state = memento.get_state();
    }

    #[allow(dead_code)]
    pub fn create_memento(&self) -> Memento {
	Memento::new(self.state.clone())
    }

    #[allow(dead_code)]
    pub fn next_state(&mut self) {
	match self.state {
	    State::Starting => self.state = State::Running,
	    State::Running => self.state = State::ShuttingDown,
	    State::ShuttingDown => println!("Done"),
	}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memento() {
	let mut originator: Originator = Originator { state: State::Starting };
	assert_eq!(State::Starting, originator.state);
	originator.next_state();
	let memento: Memento = originator.create_memento();
	assert_eq!(State::Running, originator.state);
	originator.next_state();
	assert_eq!(State::ShuttingDown, originator.state);
	originator.restore(memento);
	assert_eq!(State::Running, originator.state);
    }
}

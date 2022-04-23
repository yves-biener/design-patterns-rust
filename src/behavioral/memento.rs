#[derive(Debug, Eq, PartialEq, Clone)]
pub enum State {
    Starting,
    Running,
    ShuttingDown,
}

pub struct Memento {
    state: State,
}

impl Memento {
    pub fn new(state: State) -> Memento {
	Memento {state}
    }

    pub fn get_state(&self) -> State {
	self.state.clone()
    }
}

struct Originator {
    pub state: State,
}

impl Originator {
    pub fn restore(&mut self, memento: Memento) {
	self.state = memento.get_state();
    }

    pub fn create_memento(&self) -> Memento {
	Memento::new(self.state.clone())
    }

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

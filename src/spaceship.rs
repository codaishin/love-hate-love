#[derive(Default)]
pub struct Spaceship {
	pub name: &'static str,
	pub state: State,
}

#[derive(Default, PartialEq)]
pub enum State {
	#[default]
	UnFueled,
	Fueled,
	InSpace,
}

impl Spaceship {
	pub fn fuel(&mut self) {
		self.state = State::Fueled;
	}

	pub fn take_off(&mut self) {
		self.state = State::InSpace;
	}
}

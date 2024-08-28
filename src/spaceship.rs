#[derive(Default)]
pub struct Spaceship<TState> {
	pub name: &'static str,
	state: TState,
}

pub struct UnFueled;
pub struct Fueled;
pub struct InSpace;

impl Spaceship<UnFueled> {
	pub fn new(name: &'static str) -> Self {
		Self {
			name,
			state: UnFueled,
		}
	}

	pub fn fuel(self) -> Spaceship<Fueled> {
		Spaceship {
			name: self.name,
			state: Fueled,
		}
	}
}

impl Spaceship<Fueled> {
	pub fn take_off(self) -> Spaceship<InSpace> {
		Spaceship {
			name: self.name,
			state: InSpace,
		}
	}
}

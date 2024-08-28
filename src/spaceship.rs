use std::marker::PhantomData;

#[derive(Default)]
pub struct Spaceship<TState> {
	pub name: &'static str,
	state: PhantomData<TState>,
}

pub struct UnFueled;
pub struct Fueled;
pub struct InSpace;

impl Spaceship<UnFueled> {
	pub fn new(name: &'static str) -> Self {
		Self {
			name,
			state: PhantomData,
		}
	}

	pub fn fuel(self) -> Spaceship<Fueled> {
		Spaceship {
			name: self.name,
			state: PhantomData,
		}
	}
}

impl Spaceship<Fueled> {
	pub fn take_off(self) -> Spaceship<InSpace> {
		Spaceship {
			name: self.name,
			state: PhantomData,
		}
	}
}

impl Spaceship<InSpace> {
	pub fn land(self) -> Spaceship<UnFueled> {
		Spaceship {
			name: self.name,
			state: PhantomData,
		}
	}
}

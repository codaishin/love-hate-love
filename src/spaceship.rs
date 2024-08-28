use std::marker::PhantomData;

#[derive(Default)]
pub struct Spaceship<TState> {
	pub name: &'static str,
	hit_points: usize,
	state: PhantomData<TState>,
}

pub struct UnFueled;
pub struct Fueled;
pub struct InSpace;

fn translate<TIn, TOut>(spaceship: Spaceship<TIn>) -> Spaceship<TOut> {
	Spaceship {
		name: spaceship.name,
		hit_points: spaceship.hit_points,
		state: PhantomData,
	}
}

impl Spaceship<UnFueled> {
	pub fn new(name: &'static str) -> Self {
		Self {
			name,
			hit_points: 100,
			state: PhantomData,
		}
	}

	pub fn fuel(self) -> Spaceship<Fueled> {
		translate(self)
	}
}

impl Spaceship<Fueled> {
	pub fn take_off(self) -> Spaceship<InSpace> {
		translate(self)
	}
}

#[derive(Debug)]
pub struct Destroyed;

impl Spaceship<InSpace> {
	pub fn land(self) -> Spaceship<UnFueled> {
		translate(self)
	}

	pub fn collision_damage(mut self, damage: usize) -> Result<Spaceship<InSpace>, Destroyed> {
		if damage > self.hit_points {
			Err(Destroyed)
		} else {
			self.hit_points -= damage;
			Ok(translate(self))
		}
	}
}

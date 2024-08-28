use love_hate_love::spaceship::{Spaceship, State};

fn main() {
	let mut spaceship = Spaceship {
		name: "Defiant",
		..Default::default()
	};

	spaceship.fuel();

	if spaceship.state == State::Fueled {
		spaceship.take_off();
	}
}

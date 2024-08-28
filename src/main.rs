use love_hate_love::spaceship::Spaceship;

fn main() {
	let mut spaceship = Spaceship {
		name: "Defiant",
		..Default::default()
	};
	spaceship.fuel();
	spaceship.take_off();
}

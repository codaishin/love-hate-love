use love_hate_love::spaceship::Spaceship;

fn main() {
	let spaceship = Spaceship::new("Defiant");
	let spaceship = spaceship.fuel();
	let _spaceship = spaceship.take_off();
}

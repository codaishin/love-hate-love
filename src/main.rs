use love_hate_love::spaceship::Spaceship;

fn main() {
	let spaceship = Spaceship::new("Defiant");
	let spaceship = spaceship.fuel();
	let spaceship = spaceship.take_off();
	let _spaceship = spaceship.land();
}

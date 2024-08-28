use love_hate_love::spaceship::{Destroyed, Spaceship};

fn main() -> Result<(), Destroyed> {
	let spaceship = Spaceship::new("Defiant");
	let spaceship = spaceship.fuel();
	let spaceship = spaceship.take_off();
	let spaceship = match spaceship.collision_damage(16) {
		Ok(spaceship) => spaceship,
		Err(destroyed) => return Err(destroyed),
	};
	let _spaceship = spaceship.land();

	Ok(())
}

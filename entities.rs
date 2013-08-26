//use utils;
//mod utils;

pub mod entities {
	
	use utils::*;
	use extra::time::Timespec;

	enum EntityType {
		Hero,
		Enemy
	}

	impl EntityType {
		fn to_str(&self) -> ~str {
			match *self {
				Hero  => ~"Hero",
				Enemy => ~"Enemy",
			}
		}
	}

	pub struct Entity {
		name: ~str,
		position: @mut utils::Point,
		kind: EntityType
	}

	impl Entity {
		pub fn new_player() -> Entity {
			Entity {
				name: ~"Entity",
				position: @mut utils::Point {x: 0f, y: 0f},
				kind: Hero
			}
		}
	}

	pub trait Updatable {
		fn update(&self, dt: Timespec);
	}

	pub trait Drawable {
		fn draw(&self, dt: Timespec);
	}

	impl Updatable for Entity {
		fn update(&self, dt: Timespec) {

			match self.kind {
				_ => println(fmt!("Updating \t[%s:%s]", self.name, self.kind.to_str()))
			}			
		}
	}

	impl Drawable for Entity {
		fn draw(&self, dt: Timespec) {
			match self.kind {
				_ => println(fmt!("Drawing: \t[%s:%s]", self.name, self.kind.to_str()))
			}			
		}
	}

}
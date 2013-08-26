//use utils;
//mod utils;



pub mod entities {
	
	use utils::*;

	pub struct Player {
		name: ~str,
		position: @mut utils::Point	
	}

	impl Player {
		pub fn new_player() -> Player {
			Player {
				name: ~"Player",
				position: @mut utils::Point {x: 0f, y: 0f}
			}
		}
	}

	pub trait Entity {
		fn update(&self);
	}

	pub trait Drawable {
		fn draw(&self);
	}

	impl Entity for Player {
		fn update(&self) {
			println(fmt!("%s is thinking", self.name));
		}
	}

	impl Drawable for Player {
		fn draw(&self) {
			println(fmt!("Drawing: \t%s", self.name));
		}
	}

}
//#[link(name="roguelike", vers="0.1", author="zol")]

extern mod extra;

use std::rand::RngUtil;
use extra::time::get_time;
use extra::time::Timespec;

pub mod entities;
pub mod utils;


fn main() {	

	//  std::rand::task_rng().gen_int_range(0, 256)

	use entities::entities::*;
	use entities::*;
	use utils::*;

	let mut quit = false;

	let mut player = entities::Entity::new_player();

	let mut prevFrame = get_time();

	while !quit {
		

		let (dt, gt) = utils::getElapsedTime(prevFrame);

		if player.position.x > 100.0f {
			quit = true;
		}

		player.update(dt);
		player.draw(dt);

		prevFrame = gt;
	}
}
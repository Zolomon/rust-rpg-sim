//#[link(name="utils", vers="0.1", author="zol")]
//#[crate_type = "lib"];

pub mod utils {
	use extra::time::get_time;
	use extra::time::Timespec;

	pub fn getElapsedTime(prevFrame: Timespec) -> (Timespec, Timespec) {
		let gt = get_time();

		let dt = Timespec {
			sec: gt.sec - prevFrame.sec, 
			nsec: gt.nsec - prevFrame.nsec 
		};

		return (dt, gt);
	}

	pub struct Point {
	    x: float,
	    y: float
	}

	pub enum Direction {
	    North,
	    NorthEast,
	    East,
	    SouthEast,
	    South,
	    SouthWest,
	    West,
	    NorthWest
	}

	pub fn move(point: @mut Point, dir: Direction) -> () {

		*point = match dir {
			North 		=>	Point { x: point.x,        y: point.y + 1.0f },
			NorthEast 	=>	Point { x: point.x + 1.0f, y: point.y + 1.0f },
			East 		=>	Point { x: point.x + 1.0f, y: point.y        },
			SouthEast 	=>	Point { x: point.x + 1.0f, y: point.y - 1.0f },
			South 		=>	Point { x: point.x,        y: point.y - 1.0f },
			SouthWest 	=>	Point { x: point.x - 1.0f, y: point.y - 1.0f },
			West 		=>	Point { x: point.x - 1.0f, y: point.y        },
			NorthWest	=>	Point { x: point.x - 1.0f, y: point.y + 1.0f },
		}
	}
}
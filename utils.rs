//#[link(name="utils", vers="0.1", author="zol")]
//#[crate_type = "lib"];

pub mod utils {
	use extra::time::get_time;
	use extra::time::Timespec;
	use entities::entities::*;

	pub fn getElapsedTime(prevFrame: Timespec) -> (Timespec, Timespec) {
		let gt = get_time();

		let dt = Timespec {
			sec: gt.sec - prevFrame.sec, 
			nsec: gt.nsec - prevFrame.nsec 
		};

		return (dt, gt);
	}

	#[deriving(Eq, ToStr)]
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

	#[deriving(Eq, ToStr)]
	pub enum GoalType {
		GoalThink,
		GoalExplore,
		GoalSleep
	}

	#[deriving(Eq, ToStr)]
	pub enum GoalStatus {
		Inactive,
	    Active,
	    Completed,
	    Failed
	}

	#[deriving(Eq, ToStr)]
	struct GoalThink {
		owner: @Entity,
		status: GoalStatus,
		goal_type: GoalType
	}

	trait Goal {
		fn new(owner: @Entity) -> Self;
		
		fn activate(&self) 	{ }
		fn process(&self) 	{ }
		fn terminate(&self)	{ }
		//fn handle_msg(&self, msg: Telegram) { }
		fn add_subgoal(&self, goal: @Goal) { }

		fn is_active(&self) -> bool;
		fn is_inactive(&self) -> bool;
		fn is_completed(&self) -> bool;
		fn has_failed(&self) -> bool;
		fn get_type(&self) -> GoalType;
	}

	impl Goal for GoalThink {
		fn new(owner: @Entity) -> GoalThink { 
			GoalThink { 
				owner: owner, 
				status: Inactive,
				goal_type: GoalExplore
			}
		}	

		fn process(&self) {
			println(fmt!("Goal Think"));
		}

		fn is_active(&self) -> bool { self.status == Active }
		fn is_inactive(&self) -> bool { self.status == Inactive }
		fn is_completed(&self) -> bool { self.status == Completed }
		fn has_failed(&self) -> bool { self.status == Failed }
		fn get_type(&self) -> GoalType { self.goal_type }
	}

	enum Tree {
	    Leaf(@Goal),
	    Node(@Goal, ~[Tree])
	}

	impl Tree {
		fn push(&mut self, node: Tree) {
			match *self {
				Node(goal, ref mut children) => children.push(node),
				Leaf(_) => fail!(~"Cannot add node to leaf node")
			}
		}

		fn process(&mut self) {
			match *self {
				Node(goal, ref mut children) => {

					//for child in children.mut_iter() { child.process() }
				},

				Leaf(goal) => {
					goal.process()
				}
			}
		}
	}
}
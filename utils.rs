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
		GoalRest
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
		fn process(&self) -> GoalStatus	{ Inactive }
		fn terminate(&self)	{ }
		//fn handle_msg(&self, msg: Telegram) { }
		fn add_subgoal(&self, goal: @Goal) { }
		
		fn activate_if_inactive(&self);
		fn reactivate_if_failed(&mut self);

		fn is_active(&self) -> bool;
		fn is_inactive(&self) -> bool;
		fn is_completed(&self) -> bool;
		fn has_failed(&self) -> bool;
		fn get_type(&self) -> GoalType;
	}

	/*impl Goal for GoalThink {
		fn new(owner: @Entity) -> GoalThink { 
			GoalThink { 
				owner: owner, 
				status: Inactive,
				goal_type: GoalExplore
			}
		}	

		fn process(&self) -> GoalStatus {
			//self.activate_if_inactive();
		}

		fn is_active(&self) -> bool { self.status == Active }
		fn is_inactive(&self) -> bool { self.status == Inactive }
		fn is_completed(&self) -> bool { self.status == Completed }
		fn has_failed(&self) -> bool { self.status == Failed }
		fn get_type(&self) -> GoalType { self.goal_type }
	}*/

	#[deriving(Eq, ToStr)]
	struct GoalRest {
		owner: @Entity,
		status: GoalStatus,
		goal_type: GoalType
	}

	impl Goal for GoalRest {
		fn new(owner: @Entity) -> GoalRest { 
			GoalRest { 
				owner: owner, 
				status: Inactive,
				goal_type: GoalRest
			}
		}	

		fn activate_if_inactive(&self) { if self.is_inactive() { self.activate(); } }
		fn reactivate_if_failed(&mut self) { if self.has_failed() { self.status = Inactive; } }

		fn activate(&self) {
			println(fmt!("Activated GoalRest"));
		}

		fn process(&self) -> GoalStatus {
			println(fmt!("Processing GoalRest"));

			self.activate_if_inactive();

			return self.status
		}

		fn terminate(&self) {
			println(fmt!("Terminating GoalRest"));
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

		fn terminate(&mut self) {
			match *self {
				Node(goal, ref mut children) => for child in children.mut_iter() {
					child.terminate();
				},
				Leaf(goal) => {goal.terminate()}
			}
		}

		fn is_completed(&mut self) -> bool {
			match *self {
				Node(goal, ref mut children) => {
					let mut result = false;
					for child in children.mut_iter() {
						result |= child.is_completed();
					}
					result
				},
				Leaf(goal) => { goal.is_completed() }	
			}			
		}

		fn process(&mut self) -> GoalStatus {
			match *self {
				Node(goal, ref mut children) => {
					/*while !children.is_empty() && 
					(children.head().is_completed() || children.head().has_failed()) {
						children.head().terminate();
						children.remove(0);												
					}

					// if any subgoals remain, process the one at the front of the list
					if !children.is_empty() {
						// grab the status of the front-most subgoal
						let status_of_subgoals = children.head().process();

						// we have to test for the special case where the fron-most subgoal
						// reports "completed" nad the subgoal list contains additional goals.
						// When this is the case, to ensure the parent keeps processing its
						// subgoal list, the "active" status is returned
						if status_of_subgoals == Completed && children.len() > 1 {
							return Active;
						}
					}*/
					Completed
				},

				Leaf(goal) => {
					goal.process()
				}
			}
		}
	}
}
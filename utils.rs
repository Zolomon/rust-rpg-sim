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

	pub enum GoalType {
		GoalThink,
		GoalExplore,
		GoalSleep
	}

	pub enum GoalStatus {
		Inactive,
	    Active,
	    Completed,
	    Failed
	}

	/*pub struct GoalBase<T> {
		kind: GoalType,
		owner: @mut T,
		status: @mut GoalStatus,
	}

	impl<T> GoalBase<T> {
		pub fn new_goal<T>(owner: @mut T, goal: GoalType) -> GoalBase<T> {
			GoalBase {
				kind: goal,
				owner: owner,
				status: @mut Inactive
			}
		}
	}

	pub trait GoalTrait<T> {
		fn ActivateIfInactive(&self) -> ();
		fn ReactivateIfFailed(&mut self) -> ();
		fn Activate(&self);
		fn Process(&self) -> int;
		fn Terminate(&self);
		fn is_active(&self) -> bool;
		fn is_inactive(&self) -> bool;
		fn has_failed(&self) -> bool;
		fn get_kind(&self) -> GoalType;
		fn handle_message(&self) -> bool;
		fn add_subgoal(&self, goal: Goal<T>);
	}

	impl<T> GoalTrait<T> for GoalBase<T> {
		fn ActivateIfInactive(&self) -> () {
			if self.is_inactive() {
				self.Activate();
			}
		}
		fn ReactivateIfFailed(&mut self) -> () {
			if self.has_failed() {
				self.status = @mut Inactive;
			}
		}

		fn Activate(&self) {}
		fn Process(&self) -> int { 0}
		fn Terminate(&self) {}

		fn is_active(&self) -> bool {true}
		fn is_inactive(&self) -> bool {true}
		fn has_failed(&self) -> bool {true}

		fn get_kind(&self) -> GoalType {
			self.kind
		}

		fn handle_message(&self) -> bool {
			false
		}
		fn add_subgoal(&self, goal: Goal<T>) -> () {
			
		}
	}*/
}
fn main() {

	struct GoalExplore {
		name: ~str
	}

	struct GoalRest {
		name: ~str
	}

	trait Goal {
		fn new() -> Self;
		fn process(&self) { }
	}

	impl Goal for GoalExplore {
		fn new() -> GoalExplore {
			GoalExplore { name: ~"GoalExplore" }
		}

		fn process(&self) {
			println(fmt!("Exploring"));
		}
	}

	impl Goal for GoalRest {
		fn new() -> GoalRest {
			GoalRest { name: ~"GoalRest"}
		}

		fn process(&self) {
			println(fmt!("Resting"))
		}
	}

	enum Composite {
		GoalExplore   
	}

	let t: GoalExplore = Goal::new();
	t.process();
	let rest: GoalRest = Goal::new();
	rest.process();

	//fn foo<G: Goal>(goal: &G) { .. }
}
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
		fn to_str(&self) -> ~str;
	}

	impl Goal for GoalExplore {
		fn new() -> GoalExplore {
			GoalExplore { name: ~"GoalExplore" }
		}

		fn process(&self) {
			println(fmt!("Exploring"));
		}

		fn to_str(&self) -> ~str {
			self.name.clone()
		}
	}

	impl Goal for GoalRest {
		fn new() -> GoalRest {
			GoalRest { name: ~"GoalRest"}
		}

		fn process(&self) {
			println(fmt!("Resting"))
		}

		fn to_str(&self) -> ~str {
			self.name.clone()
		}
	}

	enum Tree {
		Leaf(@Goal),
		Node(~[Tree]),
	}

	//fn foo<G: Goal>(goal: &G) { .. }

	impl Tree {
		fn push(&mut self, node: Tree) {
			match *self {
				Node(ref mut children) => children.push(node),
				Leaf(_) => fail!(~"Cannot add node to leaf node")
			}
		}

		fn process(&mut self) {
			match *self {
				Node(ref mut children) => for x in children.mut_iter() { x.process() },
				Leaf(l) => println(fmt!("Leaf: %s", l.to_str()))
			}
		}
	}


	let mut t: @mut GoalExplore = @mut Goal::new();
	t.process();
	let mut rest: @mut GoalRest = @mut Goal::new();
	rest.process();

	let mut tree: Tree = Node(~[]);

	tree.push(Leaf(t as @Goal));

	println!("{:?}", tree);
}
fn main() {
	
	struct GoalExplore {
		name: ~str
	}

	struct GoalRest {
		name: ~str
	}

	trait Goal {
		fn new() -> Self;
		fn process(&self, depth: int) { }
		fn to_str(&self) -> ~str;
	}

	impl Goal for GoalExplore {
		fn new() -> GoalExplore { GoalExplore { name: ~"GoalExplore" } }

		fn process(&self, depth: int) { println(fmt!("Exploring")); }

		fn to_str(&self) -> ~str { self.name.clone() }
	}

	impl Goal for GoalRest {
		fn new() -> GoalRest { GoalRest { name: ~"GoalRest"} }

		fn process(&self, depth: int) { println(fmt!("Resting")) }

		fn to_str(&self) -> ~str { self.name.clone() }
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

		fn process(&mut self, depth: uint) {
			match *self {
				Node(ref mut children) => {
					println(fmt!("%sNode:", " ".repeat(depth)));
					for x in children.mut_iter() { x.process(depth+1) }},
				Leaf(l) => {
					println(fmt!("%sLeaf: %s", " ".repeat(depth), l.to_str()));
				}
			}
		}
	}

	let mut t: @mut GoalExplore = @mut Goal::new();
	let mut rest: @mut GoalRest = @mut Goal::new();

	let mut tree: Tree = Node(~[]);

	tree.push(
		Node(~[
			Leaf(t as @Goal), 
			Leaf(rest as @Goal), 
			Node(~[
				Leaf(t as @Goal),
				Leaf(t as @Goal),
				Leaf(t as @Goal),
				Node(~[
					Leaf(t as @Goal),
					Leaf(t as @Goal),
					Leaf(t as @Goal)])])]));

	tree.process(0);

	println!("{:?}", tree);
}
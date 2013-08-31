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

	enum Tree<T> {
		Leaf(T),
		Node(~[Tree<T>]),
	}

	//fn foo<G: Goal>(goal: &G) { .. }

	impl<T> Tree<T> {
		fn push(&mut self, node: Tree<T>) {
			match *self {
				Node(ref mut children) => children.push(node),
				Leaf(_) => fail!(~"Cannot add node to leaf node")
			}
		}

		/*fn to_str(&mut self) {
			match *self {
				Node(ref mut children) => for x in children.iter() { x.to_str() },
				Leaf(v) => ()//println!("{:?}", v)
			}
		}*/
	}


	let t: @GoalExplore = @Goal::new();
	t.process();
	let rest: @GoalRest = @Goal::new();
	rest.process();

	let mut tree: Tree<@Goal> = Node(~[]);

	tree.push(Leaf(t as @Goal));

	println!("{:?}", tree);
}
/*
enum Tree<T> { 
	Leaf(T), 
	Node { children: ~[Tree<T>] } 
} 

let mut my_tree: Tree<uint> = Node { children: ~[] }; 

match my_tree {     
	Node { children: ref mut children } => children.push(Leaf(1u)), 
	_ => () 
} 

my_tree  */
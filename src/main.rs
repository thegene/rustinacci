trait HasValue {
    fn get_value(&self) -> i32;
}

struct Null;

struct Node {
    value: i32,
    pointer: Null
}

impl Node {
    fn new(previous: Null) -> Node {
        Node {
            value: 0,
            pointer: previous
        }
    }
}

fn find_fib(fib_number: i32) {
    let null: Null = Null{};
    let node: Node = Node::new(null);

    println!("{}", node.value);
}

fn main() {
    find_fib(12);
}

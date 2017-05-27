trait HasValue {
    fn get_value(&self) -> i32;
}

struct Null;

impl HasValue for Null {
    fn get_value(&self) -> i32 {
        0
    }
}

struct Node<T> {
    value: i32,
    pointer: T
}

impl<T: HasValue> Node<T> {
    fn new(previous: T) -> Node<T> {
        Node {
            value: previous.get_value() + 1,
            pointer: previous
        }
    }
}

fn find_fib(fib_number: i32) {
    let null: Null = Null{};
    let node = Node::new(null);

    println!("{}", node.value);
}

fn main() {
    find_fib(12);
}

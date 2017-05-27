trait HasValue {
    fn get_value(&self) -> i32;
    fn get_previous_value(&self) -> i32;
}

struct Null;

impl HasValue for Null {
    fn get_value(&self) -> i32 {
        0
    }

    fn get_previous_value(&self) -> i32 {
        0
    }
}

struct Node<T> {
    value: i32,
    previous: T
}

impl<T: HasValue> Node<T> {
    fn new(previous: T) -> Node<T> {
        Node {
            value: Node::find_new_value(&previous),
            previous: previous
        }
    }

    fn find_new_value(previous: &T) -> i32 {
        let previous_value = previous.get_value();
        if previous_value == 0 {
            1
        } else {
            previous_value + previous.get_previous_value()
        }
    }
}

fn find_fib(fib_number: i32) {
    let head: Null = Null{};
    let node = Node::new(head);

    println!("{}", node.value);
}

fn main() {
    find_fib(12);
}

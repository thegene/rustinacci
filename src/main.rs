struct List {
    head: i32,
    tail: i32
}

impl List {
    fn new_incremented(&self) -> List {
        let new_head = match self.head {
            0 => 1,
            _ => self.head + self.tail
        };

        List {
            head: new_head,
            tail: self.head
        }
    }
}

fn find_fib(fib_number: i32) {
    let list = List {
        head: 0,
        tail: 0
    };

    let new_list = list.new_incremented();

    println!("{}", new_list.head);
}

fn main() {
    find_fib(12);
}

struct List {
    head: i32,
    tail: i32
}

impl List {
    fn increment(&self) -> List {
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

fn find_fib(mut fib_number: i32) {
    let mut list = List {
        head: 0,
        tail: 0
    };

    while fib_number > 0 {
        list = list.increment();
        fib_number -= 1;
    }

    println!("{}", list.head);
}

fn main() {
    find_fib(12);
}

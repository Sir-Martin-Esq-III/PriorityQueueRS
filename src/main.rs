use pq::{Node, PriorityQueue};

fn main() {
    let init = Node {
        key: 10,
        value: "hello",
    };

    let mut queue = PriorityQueue {
        queue: vec![init],
        heap_size: 1,
    };

    queue.insert(Node {
        key: 12,
        value: "world",
    });

    queue.insert(Node {
        key: 20,
        value: "foo",
    });

    queue.insert(Node {
        key: 8,
        value: "bar",
    });

    queue.insert(Node {
        key: 1,
        value: "baz",
    });

    loop {
        let max = queue.extract_max();

        match max {
            Some(max) => {
                println!("{}", max)
            }
            _ => break,
        }
    }
}

use pq::{Node, PriorityQueue};

fn main() {
    let init = Node {
        key: 10,
        value: "hello",
    };

    let mut queueeueueue = PriorityQueue {
        queue: vec![init],
        heap_size: 1,
    };

    queueeueueue.insert(Node {
        key: 12,
        value: "world",
    });

    queueeueueue.insert(Node {
        key: 20,
        value: "foo",
    });

    queueeueueue.insert(Node {
        key: 8,
        value: "bar",
    });

    queueeueueue.insert(Node {
        key: 1,
        value: "baz",
    });

    loop {
        let max = queueeueueue.extract_max();

        match max {
            Some(max) => {
                println!("{}", max)
            }
            _ => break,
        }
    }
}

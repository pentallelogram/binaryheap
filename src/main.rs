//#![allow(incomplete_features)]
//#![feature (const_generics)]

struct BinaryHeap {
    // The underlying data of the binary heap
    data: Vec<u32>,
}

impl BinaryHeap {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn insert(&mut self, val: u32) {
        // where the value currently is
        let mut idx = self.data.len();

        // push the value to the end of the array.
        self.data.push(val);

        loop {
            // break if inserting at the root
            if idx == 0 { break; }

            // get the index of the parent
            let parent = (idx - 1 ) / 2;

            if val < self.data[parent] {
                self.data.swap(parent, idx);
                idx = parent;
            } else {
                break;
            }
        }
    }

    fn pop_min(&mut self) -> Option<u32> {

        if let Some(mut ret) = self.data.pop() {
            // swap the last thing with the first
            std::mem::swap(&mut self.data[0], &mut ret);

            // index of the value we're pushing down
            let mut idx = 0;
            Some(0)
        } else {
            None
        }

    }
}

fn main() {
    let mut bh = BinaryHeap::new();
    bh.insert(5); print!("{:?}\n", bh.data);
    bh.insert(10); print!("{:?}\n", bh.data);
    bh.insert(5); print!("{:?}\n", bh.data);
    bh.insert(5); print!("{:?}\n", bh.data);
    println!("Hello, world!");
}

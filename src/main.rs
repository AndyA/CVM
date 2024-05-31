extern crate fastrand;
use std::collections::HashSet;

#[derive(Debug)]
struct CVM<T> {
    size: usize,
    seen: HashSet<T>,
    mask: usize,
}

impl<T: std::hash::Hash + std::cmp::Eq + Copy> CVM<T> {
    pub fn new(size: usize) -> CVM<T> {
        CVM {
            size,
            seen: HashSet::new(),
            mask: 0,
        }
    }

    fn halve(&mut self) {
        // This doesn't happen very often (O(log2 N)), so it's not worth optimizing much.
        let mut seen: Vec<T> = self.seen.drain().collect();
        fastrand::shuffle(&mut seen);
        self.seen = seen[0..self.size / 2].iter().cloned().collect();
        self.mask = self.mask << 1 | 1;
    }

    pub fn count_item(&mut self, item: T) {
        if (fastrand::usize(..) & self.mask) != 0 {
            self.seen.remove(&item);
        } else {
            // Bizarrely this is faster if we check membership explicitly even
            // though insert() must implicitly do the same check internally.
            if !self.seen.contains(&item) {
                self.seen.insert(item);
                // Only consider halving if the set has grown - which happens increasingly
                // rarely as the cardinality increases and the mask gets wider
                if self.seen.len() >= self.size {
                    self.halve();
                }
            }
        }
    }

    pub fn cardinality(&self) -> usize {
        self.seen.len() * (self.mask + 1)
    }
}

fn main() {
    let mut cvm: CVM<i32> = CVM::new(16384);
    for i in 0..50_000_000 {
        cvm.count_item(i);
    }
    println!("rust: {}", cvm.cardinality());

    let mut cvm2: CVM<i32> = CVM::new(16384);
    for i in 0..50_000_000 {
        cvm2.count_item(i % 1000);
    }
    println!("rust: {}", cvm2.cardinality());
}

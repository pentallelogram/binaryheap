#![feature(box_syntax)]
#![feature(new_uninit)]
use std::collections::BinaryHeap;
//#![allow(incomplete_features)]
//#![feature (const_generics)]
#[derive(Debug, Clone, Copy)]
pub struct PrngTrio {
    xstate: u64,
    ystate: u64,
    zstate: u64,
}

impl PrngTrio {
    pub fn new(xstate: u64, ystate: u64, zstate: u64) -> Self {
        return Self { xstate, ystate, zstate };
    }

    pub fn range(&mut self, min: usize, max: usize) -> usize {
        return ((self.next_u64() as usize) % (max - min)) + min;
    }

    pub fn new_from_u64(seed: u64) -> Self {
        let mut res = Self::new(seed, seed^0xec77152282650854, seed^0xb377f6512e582538);
        for _ in 0..4{
            res.next_u64();
        }
        return res;
    }

    pub fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    pub fn next_u64(&mut self) -> u64 {
        let xp = self.xstate;
        let yp = self.ystate;
        let zp = self.zstate;

        self.xstate = 15241094284759029579u64.wrapping_mul(zp);

        self.ystate = yp.wrapping_sub(xp);
        self.ystate = self.ystate.rotate_left(12);

        self.zstate = zp.wrapping_sub(yp);
        self.zstate = self.zstate.rotate_left(44);

        return xp;
    }
}

struct BinaryHeapOurs {
    // The underlying data of the binary heap
    data: Vec<u32>,
}

impl BinaryHeapOurs {
    fn new() -> Self {
        Self { data: Vec::with_capacity(1024 * 1024) }
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

            if val > self.data[parent] {
                self.data.swap(parent, idx);
                idx = parent;
            } else {
                break;
            }
        }
    }

    fn pop_min(&mut self) -> Option<u32> {

        if let Some(mut ret) = self.data.pop() {
            // can't remove from an empty heap
            if self.data.len() == 0 { return Some(ret); }

            // swap the last thing with the first
            std::mem::swap(&mut self.data[0], &mut ret);

            // index of the value we're pushing down
            let mut idx = 0;
            loop {
                let lefti  = idx * 2 + 1;
                let righti = idx * 2 + 2;
                let left   =  self.data.get(lefti).copied();
                let right  = self.data.get(righti).copied();

                let (smalli, smallv) = if left.is_none() && right.is_none() {
                    // No more bubbling
                    break;
                } else if right.is_none() {
                    // Only check the left
                    (lefti, left.unwrap())
                } else {
                    if left > right {
                        (lefti, left.unwrap())
                    } else {
                        (righti, right.unwrap())
                    }
                };

                if smallv > self.data[idx] {
                    self.data.swap(smalli, idx);
                    idx = smalli;
                } else {
                    // We reached the end of bubbling down
                    break;
                }

            }
            Some(ret)
        } else {
            None
        }

    }
}

struct BinaryHeapFast {
    // The underlying data of the binary heap
    data: Box<[u32; 1024*1024]>,
    in_use: usize,
}

impl BinaryHeapFast {
    fn new() -> Self {
        let contents = Box::<[u32; 1024*1024]>::new_zeroed();
        let contents = unsafe { contents.assume_init() };
        Self { data: contents, in_use: 0 }
    }

    fn insert(&mut self, val: u32) {
        /*
        // where the value currently is
        let mut idx = self.data.len();

        // push the value to the end of the array.
        self.data.push(val);

        loop {
            // break if inserting at the root
            if idx == 0 { break; }

            // get the index of the parent
            let parent = (idx - 1 ) / 2;

            if val > self.data[parent] {
                self.data.swap(parent, idx);
                idx = parent;
            } else {
                break;
            }
        }
        */
    }

    fn pop_min(&mut self) -> Option<u32> {
        None
    /*

        if let Some(mut ret) = self.data.pop() {
            // can't remove from an empty heap
            if self.data.len() == 0 { return Some(ret); }

            // swap the last thing with the first
            std::mem::swap(&mut self.data[0], &mut ret);

            // index of the value we're pushing down
            let mut idx = 0;
            loop {
                let lefti  = idx * 2 + 1;
                let righti = idx * 2 + 2;
                let left   =  self.data.get(lefti).copied();
                let right  = self.data.get(righti).copied();

                let (smalli, smallv) = if left.is_none() && right.is_none() {
                    // No more bubbling
                    break;
                } else if right.is_none() {
                    // Only check the left
                    (lefti, left.unwrap())
                } else {
                    if left > right {
                        (lefti, left.unwrap())
                    } else {
                        (righti, right.unwrap())
                    }
                };

                if smallv > self.data[idx] {
                    self.data.swap(smalli, idx);
                    idx = smalli;
                } else {
                    // We reached the end of bubbling down
                    break;
                }

            }
            Some(ret)
        } else {
            None
        }

    */
    }
}

fn rdtsc() -> u64 {
    unsafe {
        std::arch::x86_64::_rdtsc()
    }
}

fn main() {
    let mut rng = PrngTrio::new(0xffa0c2084804ba2a, 0xe96fa5c59f4c40a, 0xf68b900e4440eceb);
    let mut obh = BinaryHeapOurs::new();
    let mut fbh = BinaryHeapFast::new();
    let mut rbh = BinaryHeap::with_capacity(1024 * 1024);

    let iters = 100_000;

    for size in 1..10_000{
        let mut rng_copy = rng.clone();
        let mut rng_fast = rng.clone();

        let it = rdtsc();
        for _ in 0..iters {
            obh.data.clear();

            for _ in 0..size {
                obh.insert(rng.next_u32());
            }
            while let Some(_val) = obh.pop_min() {}
        }
        let elapsed = (rdtsc() - it) as f64;
        let our_perf = elapsed / iters as f64 / size as f64;

        let it = rdtsc();
        for _ in 0..iters {
            rbh.clear();

            for _ in 0..size {
                rbh.push(rng_copy.next_u32());
            }
            while let Some(_val) = rbh.pop() {}
        }
        let elapsed = (rdtsc() - it) as f64;
        let rust_perf = elapsed / iters as f64 / size as f64;

        let it = rdtsc();
        for _ in 0..iters {
            fbh.in_use = 0;
            //fbh.data.clear();

            for _ in 0..size {
                fbh.insert(rng_fast.next_u32());
            }
            while let Some(_val) = fbh.pop_min() {}
        }
        let elapsed = (rdtsc() - it) as f64;
        let fast_perf = elapsed / iters as f64 / size as f64;

        println!("{:10} {:12.6} {:12.6} {:12.6}", size, our_perf, rust_perf, fast_perf);
    }
}

use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(usize);

impl Id {
    pub fn inner(&self) -> usize {
        self.0
    }
}

impl From<usize> for Id {
    fn from(id: usize) -> Self {
        Id(id)
    }
}

#[derive(Debug)]
pub struct SequentialIdAllocator {
    now: AtomicUsize,
}

impl Default for SequentialIdAllocator {
    fn default() -> Self {
        Self::new()
    }
}

impl SequentialIdAllocator {
    pub fn new() -> Self {
        SequentialIdAllocator {
            now: Default::default(),
        }
    }

    pub fn allocate(&self) -> Id {
        self.now.fetch_add(1, Ordering::Relaxed).into()
    }
}

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

#[derive(Debug, Default)]
pub struct Counter(u64);

impl Counter {
    pub fn add(&mut self, value: u64) { self.0 += value; }
    pub fn value(&self) -> u64 { self.0 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn accumulates() {
        let mut c = Counter::default();
        c.add(3);
        c.add(4);
        assert_eq!(c.value(), 7);
    }
}
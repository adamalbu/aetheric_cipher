#[derive(Debug, PartialEq)]
pub struct Exponential {
    pub base: f64,
    pub growth_rate: f64,
}

impl Exponential {
    pub fn new(base: f64, growth_rate: f64) -> Self {
        Self { base, growth_rate }
    }
    pub fn calculate(&self, multiplier: f64) -> f64 {
        self.base + (self.growth_rate * multiplier)
    }
}

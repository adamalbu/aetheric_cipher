use std::fmt;

pub struct Producer {
    pub name: String,
    pub flux_per_second: f64,
    upgrade_cost: Box<dyn Fn(f64) -> f64>,
}

impl PartialEq for Producer {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.flux_per_second == other.flux_per_second
    }
}

impl fmt::Debug for Producer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Producer")
            .field("name", &self.name)
            .field("flux_per_second", &self.flux_per_second)
            .field("upgrade_cost", &"Box<dyn Fn(f64) -> f64>") // Placeholder for the closure
            .finish()
    }
}

impl Producer {
    pub fn new(name: String, nodes_per_second: f64, upgrade_cost: Box<dyn Fn(f64) -> f64>) -> Self {
        Self {
            name,
            flux_per_second: nodes_per_second,
            upgrade_cost,
        }
    }

    pub fn get_upgrade_cost(&self) -> f64 {
        (self.upgrade_cost)(self.flux_per_second)
    }

    pub fn upgrade(&mut self) {
        self.flux_per_second += 2.0;
    }
}

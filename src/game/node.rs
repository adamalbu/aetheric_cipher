#[derive(Debug, PartialEq)]
pub struct Cost {
    pub base: f64,
    pub growth_rate: f64,
}

impl Cost {
    pub fn new(base: f64, growth_rate: f64) -> Self {
        Self { base, growth_rate }
    }

    pub fn get_cost(&self, multiplier: f64) -> f64 {
        self.base + (self.growth_rate * multiplier)
    }
}

#[derive(Debug, PartialEq)]
pub struct FluxPerSecond {
    pub base: f64,
    pub growth_rate: f64,
}

impl FluxPerSecond {
    pub fn new(base: f64, growth_rate: f64) -> Self {
        Self { base, growth_rate }
    }

    pub fn get_flux_per_second(&self, multiplier: f64) -> f64 {
        self.base + (self.growth_rate * multiplier)
    }
}

#[derive(Debug, PartialEq)]
pub struct Node {
    pub name: String,
    pub cost: Cost,
    pub flux_per_second: FluxPerSecond,
    pub level: u32,
}

impl Node {
    pub fn new_from_rates(
        name: String,
        base_flux_per_second: f64,
        flux_per_second_growth_rate: f64,
        base_cost: f64,
        cost_growth_rate: f64,
    ) -> Self {
        let flux_per_second = FluxPerSecond::new(base_flux_per_second, flux_per_second_growth_rate);
        let cost = Cost::new(base_cost, cost_growth_rate);

        Self {
            name,
            flux_per_second,
            cost,
            level: 0,
        }
    }

    pub fn new(name: String, cost: Cost, flux_per_second: FluxPerSecond) -> Self {
        Self {
            name,
            cost,
            flux_per_second,
            level: 0,
        }
    }

    pub fn get_cost(&self) -> f64 {
        self.cost.get_cost(self.level as f64)
    }

    pub fn get_flux_per_second(&self) -> f64 {
        self.flux_per_second.get_flux_per_second(self.level as f64)
    }

    pub fn upgrade(&mut self) {
        self.level += 1;
    }
}

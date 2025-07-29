use crate::game::equations::Exponential;

#[derive(Debug, PartialEq)]
pub struct Nexus {
    pub cost: Exponential,
    pub base_flux_per_second: Exponential,
    pub multiplier: f64,
    pub multiplier_increase_per_second: f64,
    pub level: u32,
}

impl Nexus {
    pub fn new_from_rates(
        base_flux_per_second: f64,
        flux_per_second_growth_rate: f64,
        base_cost: f64,
        cost_growth_rate: f64,
    ) -> Self {
        let flux_per_second = Exponential::new(base_flux_per_second, flux_per_second_growth_rate);
        let cost = Exponential::new(base_cost, cost_growth_rate);

        Self {
            base_flux_per_second: flux_per_second,
            cost,
            multiplier: 1.0,
            multiplier_increase_per_second: 0.0,
            level: 0,
        }
    }

    #[allow(dead_code)]
    pub fn new(cost: Exponential, flux_per_second: Exponential) -> Self {
        let cost = cost;
        let flux_per_second = flux_per_second;

        Self {
            cost,
            base_flux_per_second: flux_per_second,
            multiplier: 1.0,
            multiplier_increase_per_second: 0.0,
            level: 0,
        }
    }

    pub fn get_cost(&self) -> f64 {
        self.cost.calculate(self.level as f64)
    }

    pub fn get_flux_per_second(&self) -> f64 {
        self.base_flux_per_second
            .calculate(self.get_total_multiplier() as f64)
    }

    fn get_total_multiplier(&self) -> f64 {
        self.multiplier * self.level as f64
    }

    pub fn upgrade(&mut self) {
        self.level += 1;
    }
}

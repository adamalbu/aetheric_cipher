use crate::game::Nexus;
use std::time::Duration;

#[derive(Debug)]
pub struct GameManager {
    pub flux: f64,
    pub nexus: Nexus,
    // pub nodes: HashMap<String, Nexus>,
}

impl GameManager {
    pub fn new_default() -> Self {
        let nexus = Nexus::new_from_rates(
            0.0,  // base_flux_per_second
            0.02, // flux_per_second_growth_rate
            0.0,  // base_cost
            1.05,
        );
        Self { flux: 10.0, nexus }
    }

    #[allow(dead_code)]
    pub fn new(starting_flux: f64, nexus: Nexus) -> Self {
        Self {
            flux: starting_flux,
            nexus,
        }
    }

    pub fn tick(&mut self, dt: Duration) {
        let dt_secs = dt.as_millis() as f64 / 1000.0;

        self.flux += self.nexus.get_flux_per_second() * dt_secs;
    }

    pub fn upgrade_nexus(&mut self) -> Result<(), String> {
        let upgrade_cost = self.nexus.get_cost();

        if self.flux >= upgrade_cost {
            self.nexus.upgrade();
            self.flux -= upgrade_cost;
            Ok(())
        } else {
            Err("Not enough flux".into())
        }
    }
}

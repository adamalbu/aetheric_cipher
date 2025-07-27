use crate::game::Producer;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug)]
pub struct GameState {
    pub flux: f64,
    pub producers: HashMap<String, Producer>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            flux: 0.0,
            producers: HashMap::new(),
        }
    }

    pub fn tick(&mut self, dt: Duration) {
        let dt_secs = dt.as_millis() as f64 / 1000.0;

        for (_id, producer) in &self.producers {
            self.flux += producer.flux_per_second * dt_secs;
        }
    }

    pub fn upgrade_producer(&mut self, id: &str) -> Result<(), String> {
        let producer = self
            .producers
            .get_mut(id)
            .ok_or(&format!("Producer {} not found", id))?;

        let upgrade_cost = producer.get_upgrade_cost();

        if self.flux >= upgrade_cost {
            producer.upgrade();
            self.flux -= upgrade_cost;
            Ok(())
        } else {
            Err("Not enough flux".into())
        }
    }
}

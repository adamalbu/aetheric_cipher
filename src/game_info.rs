use std::fmt;
use std::{collections::HashMap, time::Duration};

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

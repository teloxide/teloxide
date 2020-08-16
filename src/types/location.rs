use serde::{Deserialize, Serialize};

/// This object represents a point on the map.
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Location {
    /// Longitude as defined by sender.
    pub longitude: f64,

    /// Latitude as defined by sender.
    pub latitude: f64,
}

impl Location {
    pub fn new(longitude: f64, latitude: f64) -> Self {
        Self { longitude, latitude }
    }

    pub fn latitude(mut self, val: f64) -> Self {
        self.latitude = val;
        self
    }

    pub fn longitude(mut self, val: f64) -> Self {
        self.longitude = val;
        self
    }
}

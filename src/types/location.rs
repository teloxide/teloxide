use serde::{Deserialize, Serialize};

/// This object represents a point on the map.
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Location {
    /// Longitude as defined by sender.
    pub longitude: f64,

    /// Latitude as defined by sender.
    pub latitude: f64,
}

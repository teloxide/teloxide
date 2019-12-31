use serde::{Deserialize, Serialize};

/// This object represents a point on the map.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Location {
    /// Longitude as defined by sender.
    pub longitude: f64,

    /// Latitude as defined by sender.
    pub latitude: f64,
}

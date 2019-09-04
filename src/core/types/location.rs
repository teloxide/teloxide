use serde::{Deserialization, Serialization};

#[derive(Debug, Serialization, Deserialization, Clone)]
/// This object represents a point on the map.
struct Location {
    /// Longitude as defined by sender
    pub longitude: f64,
    /// Latitude as defined by sender
    pub latitude: f64,
}
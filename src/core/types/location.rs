use serde::{Serialization, Deserialization};

#[derive(Debug, Serialization, Deserialization)]
/// This object represents a point on the map.
struct Location {
    /// Longitude as defined by sender
    pub longitude: f64,
    /// Latitude as defined by sender
    pub latitude: f64,
}
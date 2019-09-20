#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
/// This object represents a point on the map.
pub struct Location {
    /// Longitude as defined by sender
    pub longitude: f64,
    /// Latitude as defined by sender
    pub latitude: f64,
}

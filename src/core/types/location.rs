use serde::{Serialization, Deserialization};

#[derive(Debug, Serialization, Deserialization)]
struct Location {
    longitude: f64,
    latitude: f64,
}
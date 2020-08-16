use serde::{Deserialize, Serialize};

use crate::types::Location;

/// This object represents a venue.
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Venue {
    /// Venue location.
    pub location: Location,

    /// Name of the venue.
    pub title: String,

    /// Address of the venue.
    pub address: String,

    /// Foursquare identifier of the venue.
    pub foursquare_id: Option<String>,

    /// Foursquare type of the venue. (For example,
    /// `arts_entertainment/default`, `arts_entertainment/aquarium` or
    /// `food/icecream`.)
    pub foursquare_type: Option<String>,
}

impl Venue {
    pub fn new<S1, S2>(location: Location, title: S1, address: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            location,
            title: title.into(),
            address: address.into(),
            foursquare_id: None,
            foursquare_type: None,
        }
    }

    pub fn title<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.title = val.into();
        self
    }

    pub fn address<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.address = val.into();
        self
    }

    pub fn foursquare_id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.foursquare_id = Some(val.into());
        self
    }

    pub fn foursquare_type<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.foursquare_type = Some(val.into());
        self
    }
}

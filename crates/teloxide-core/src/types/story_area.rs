use serde::{Deserialize, Serialize};

use crate::types::{Argb, CountryCode, ReactionType};

/// Describes a clickable area on a story media.
#[derive(Clone, Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct StoryArea {
    /// Position of the area
    pub position: StoryAreaPosition,

    /// Type of the area
    pub r#type: StoryAreaType,
}

/// Describes the position of a clickable area within a story.
#[derive(Clone, Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct StoryAreaPosition {
    /// The abscissa of the area's center, as a percentage of the media width
    pub x_percentage: f64,

    /// The ordinate of the area's center, as a percentage of the media height
    pub y_percentage: f64,

    /// The width of the area's rectangle, as a percentage of the media width
    pub width_percentage: f64,

    /// The height of the area's rectangle, as a percentage of the media height
    pub height_percentage: f64,

    /// The clockwise rotation angle of the rectangle, in degrees; 0-360
    pub rotation_angle: f64,

    /// The radius of the rectangle corner rounding, as a percentage of the
    /// media width
    pub corner_radius_percentage: f64,
}

/// Describes the physical address of a location.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct LocationAddress {
    /// The two-letter ISO 3166-1 alpha-2 country code of the country where the
    /// location is located
    pub country_code: CountryCode,

    /// State of the location
    pub state: Option<String>,

    /// City of the location
    pub city: Option<String>,

    /// Street address of the location
    pub street: Option<String>,
}

/// Describes the type of a clickable area on a story.
#[derive(Clone, Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum StoryAreaType {
    Location(StoryAreaTypeLocation),
    SuggestedReaction(StoryAreaTypeSuggestedReaction),
    Link(StoryAreaTypeLink),
    Weather(StoryAreaTypeWeather),
    UniqueGift(StoryAreaTypeUniqueGift),
}

/// Describes a story area pointing to a location. Currently, a story can have
/// up to 10 location areas.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct StoryAreaTypeLocation {
    /// Location latitude in degrees
    pub latitude: f64,

    /// Location longitude in degrees
    pub longitude: f64,

    /// Address of the location
    pub address: Option<LocationAddress>,
}

/// Describes a story area pointing to a suggested reaction. Currently, a story
/// can have up to 5 suggested reaction areas.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct StoryAreaTypeSuggestedReaction {
    /// Type of the reaction
    pub reaction_type: ReactionType,

    /// Pass _true_ if the reaction area has a dark background
    pub is_dark: Option<bool>,

    /// Pass _true_ if reaction area corner is flipped
    pub is_flipped: Option<bool>,
}

/// Describes a story area pointing to an HTTP or tg:// link. Currently, a story
/// can have up to 3 link areas.
#[derive(Clone, Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct StoryAreaTypeLink {
    /// HTTP or tg:// URL to be opened when the area is clicked
    pub url: reqwest::Url,
}

/// Describes a story area containing weather information. Currently, a story
/// can have up to 3 weather areas.
#[derive(Clone, Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct StoryAreaTypeWeather {
    /// Temperature, in degree Celsius
    pub temperature: f64,

    /// Emoji representing the weather
    pub emoji: String,

    /// A color of the area background in the ARGB format
    pub background_color: Argb,
}

/// Describes a story area pointing to a unique gift. Currently, a story can
/// have at most 1 unique gift area.
#[derive(Clone, Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct StoryAreaTypeUniqueGift {
    /// Unique name of the gift
    pub name: String,
}

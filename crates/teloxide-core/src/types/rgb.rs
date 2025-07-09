use rgb::RGB8;
use serde::{de::Visitor, Deserialize, Serialize};

/// RGB color format
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    /// Convert a [`Rgb`] struct into a big endian `u32` representing the RGB
    /// color.
    ///
    /// # Example
    ///
    /// ```
    /// use teloxide_core::types::Rgb;
    /// assert_eq!(Rgb { r: 0xAA, g: 0xBB, b: 0xCC }.to_u32(), 0xAABBCC);
    /// ```
    ///
    /// [`Rgb`]: Rgb
    pub fn to_u32(self) -> u32 {
        u32::from_be_bytes([0, self.r, self.g, self.b])
    }

    /// Convert a big endian `u32` representing the RGB color into a [`Rgb`]
    /// struct.
    ///
    /// # Example
    ///
    /// ```
    /// use teloxide_core::types::Rgb;
    /// assert_eq!(Rgb::from_u32(0xAABBCC), Rgb { r: 0xAA, g: 0xBB, b: 0xCC });
    /// ```
    ///
    /// [`Rgb`]: Rgb
    pub fn from_u32(rgb: u32) -> Self {
        let [_, r, g, b] = rgb.to_be_bytes();
        Rgb { r, g, b }
    }
}

impl Serialize for Rgb {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.to_u32())
    }
}

impl<'de> Deserialize<'de> for Rgb {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct V;

        impl Visitor<'_> for V {
            type Value = Rgb;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an integer represeting an RGB color")
            }

            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::from_u32(v))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_u32(v.try_into().map_err(|_| E::custom("rgb value doesn't fit u32"))?)
            }
        }

        deserializer.deserialize_u32(V)
    }
}

impl From<RGB8> for Rgb {
    fn from(color: RGB8) -> Self {
        Rgb { r: color.r, g: color.g, b: color.b }
    }
}

/// ARGB color format
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Argb {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Argb {
    /// Convert a [`Argb`] struct into a big endian `u32` representing the ARGB
    /// color.
    ///
    /// # Example
    ///
    /// ```
    /// use teloxide_core::types::Argb;
    /// assert_eq!(Argb { a: 0x11, r: 0xAA, g: 0xBB, b: 0xCC }.to_u32(), 0x11AABBCC);
    /// ```
    ///
    /// [`Argb`]: Argb
    pub fn to_u32(self) -> u32 {
        u32::from_be_bytes([self.a, self.r, self.g, self.b])
    }

    /// Convert a big endian `u32` representing the ARGB color into a [`Argb`]
    /// struct.
    ///
    /// # Example
    ///
    /// ```
    /// use teloxide_core::types::Argb;
    /// assert_eq!(Argb::from_u32(0x11AABBCC), Argb { a: 0x11, r: 0xAA, g: 0xBB, b: 0xCC });
    /// ```
    ///
    /// [`Argb`]: Argb
    pub fn from_u32(argb: u32) -> Self {
        let [a, r, g, b] = argb.to_be_bytes();
        Argb { a, r, g, b }
    }
}

impl Serialize for Argb {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.to_u32())
    }
}

impl<'de> Deserialize<'de> for Argb {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct V;

        impl Visitor<'_> for V {
            type Value = Argb;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an integer represeting an ARGB color")
            }

            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::from_u32(v))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_u32(v.try_into().map_err(|_| E::custom("argb value doesn't fit u32"))?)
            }
        }

        deserializer.deserialize_u32(V)
    }
}

impl From<rgb::Argb<u8>> for Argb {
    fn from(color: rgb::Argb<u8>) -> Self {
        Argb { a: color.a, r: color.r, g: color.g, b: color.b }
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[test]
    fn rgb() {
        #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
        struct Struct {
            color: Rgb,
        }

        let json = format!(r#"{{"color":{}}}"#, 0x00AABBCC);
        let Struct { color } = serde_json::from_str(&json).unwrap();

        assert_eq!(color, Rgb { r: 0xAA, g: 0xBB, b: 0xCC })
    }

    #[test]
    fn argb() {
        #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
        struct Struct {
            color: Argb,
        }

        let json = format!(r#"{{"color":{}}}"#, 0x11AABBCC);
        let Struct { color } = serde_json::from_str(&json).unwrap();

        assert_eq!(color, Argb { a: 0x11, r: 0xAA, g: 0xBB, b: 0xCC })
    }
}

use femtovg::Color;
use serde::ser::SerializeStruct;

#[derive(Clone, PartialEq, Debug)]
pub struct Theme {
    pub background: ColorAndText,
    pub layer_direction: ColorDirection,
    pub primary: ColorAndText,
    pub primary_variant: ColorAndText,
    pub secondary: ColorAndText,
    pub error: ColorAndText,
}

impl Default for Theme {
    fn default() -> Self {
        match dark_light::detect() {
            dark_light::Mode::Light => Self::light(),
            _ => Self::dark(),
        }
    }
}

#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct ColorAndText {
    pub color: Color,
    pub text: Color,
}

impl Theme {
    // based on https://m2.material.io/design/color/dark-theme.html
    pub fn dark() -> Theme {
        Theme {
            background: ColorAndText {
                color: Color::rgb(0x12, 0x12, 0x12),
                text: Color::rgb(0xFF, 0xFF, 0xFF),
            },
            layer_direction: ColorDirection::Lighten,
            primary: ColorAndText {
                color: Color::rgb(0xBB, 0x86, 0xFC),
                text: Color::rgb(0, 0, 0),
            },
            primary_variant: ColorAndText {
                color: Color::rgb(0x37, 0x00, 0xB3),
                text: Color::rgb(0xFF, 0xFF, 0xFF),
            },
            secondary: ColorAndText {
                color: Color::rgb(0x03, 0xDA, 0xC6),
                text: Color::rgb(0, 0, 0),
            },
            error: ColorAndText {
                color: Color::rgb(0xCF, 0x66, 0x79),
                text: Color::rgb(0, 0, 0),
            },
        }
    }

    pub fn light() -> Theme {
        Theme {
            background: ColorAndText {
                color: Color::rgb(0xFF, 0xFF, 0xFF),
                text: Color::rgb(0, 0, 0),
            },
            layer_direction: ColorDirection::Darken,
            primary: ColorAndText {
                color: Color::rgb(0x62, 0x00, 0xEE),
                text: Color::rgb(0xFF, 0xFF, 0xFF),
            },
            primary_variant: ColorAndText {
                color: Color::rgb(0x37, 0x00, 0xB3),
                text: Color::rgb(0xFF, 0xFF, 0xFF),
            },
            secondary: ColorAndText {
                color: Color::rgb(0x03, 0xDA, 0xC6),
                text: Color::rgb(0, 0, 0),
            },
            error: ColorAndText {
                color: Color::rgb(0xB0, 0x00, 0x20),
                text: Color::rgb(0xFF, 0xFF, 0xFF),
            },
        }
    }

    // pub fn layer_color(&self, layer: usize) -> Color {
    //     self.layer_direction.apply(self.background.color, layer)
    // }
}

impl serde::ser::Serialize for Theme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if self == &Self::dark() {
            serializer.serialize_str("dark")
        } else if self == &Self::light() {
            serializer.serialize_str("light")
        } else {
            let mut s = serializer.serialize_struct("theme", 6)?;
            s.serialize_field("background", &self.background)?;
            s.serialize_field("layer_direction", &self.layer_direction)?;
            s.serialize_field("primary", &self.primary)?;
            s.serialize_field("primary_variant", &self.primary_variant)?;
            s.serialize_field("secondary", &self.secondary)?;
            s.serialize_field("error", &self.error)?;
            s.end()
        }
    }
}

impl<'de> serde::de::Deserialize<'de> for Theme {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ThemeVisitor;

        impl<'de> serde::de::Visitor<'de> for ThemeVisitor {
            type Value = Theme;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a theme name 'dark' or 'light', or a theme object")
            }

            fn visit_str<E>(self, value: &str) -> Result<Theme, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "dark" => Ok(Theme::dark()),
                    "light" => Ok(Theme::light()),
                    _ => Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(value),
                        &"dark or light",
                    )),
                }
            }

            fn visit_map<A>(self, mut map: A) -> Result<Theme, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut background = None;
                let mut layer_direction = None;
                let mut primary = None;
                let mut primary_variant = None;
                let mut secondary = None;
                let mut error = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "background" => {
                            background = Some(map.next_value()?);
                        }
                        "layer_direction" => {
                            layer_direction = Some(map.next_value()?);
                        }
                        "primary" => {
                            primary = Some(map.next_value()?);
                        }
                        "primary_variant" => {
                            primary_variant = Some(map.next_value()?);
                        }
                        "secondary" => {
                            secondary = Some(map.next_value()?);
                        }
                        "error" => {
                            error = Some(map.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                key,
                                &[
                                    "background",
                                    "layer_direction",
                                    "primary",
                                    "primary_variant",
                                    "secondary",
                                    "error",
                                ],
                            ));
                        }
                    }
                }

                Ok(Theme {
                    background: background
                        .ok_or_else(|| serde::de::Error::missing_field("background"))?,
                    layer_direction: layer_direction
                        .ok_or_else(|| serde::de::Error::missing_field("layer_direction"))?,
                    primary: primary.ok_or_else(|| serde::de::Error::missing_field("primary"))?,
                    primary_variant: primary_variant
                        .ok_or_else(|| serde::de::Error::missing_field("primary_variant"))?,
                    secondary: secondary
                        .ok_or_else(|| serde::de::Error::missing_field("secondary"))?,
                    error: error.ok_or_else(|| serde::de::Error::missing_field("error"))?,
                })
            }
        }

        deserializer.deserialize_any(ThemeVisitor)
    }
}

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum ColorDirection {
    Darken,
    Lighten,
}

impl ColorDirection {
    #[allow(dead_code)]
    const fn apply(self, color: Color, times: usize) -> Color {
        match self {
            ColorDirection::Darken => {
                let mut hsl = Hsla::new(color);
                hsl.l -= 0.08 * times as f32;
                if hsl.l < 0.0 {
                    hsl.l = 0.0;
                }
                hsl.to_color()
            }
            ColorDirection::Lighten => {
                let mut hsl = Hsla::new(color);
                hsl.l += 0.08 * times as f32;
                if hsl.l > 1.0 {
                    hsl.l = 1.0;
                }
                hsl.to_color()
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Hsla {
    h: f32,
    s: f32,
    l: f32,
    a: f32,
}

impl Hsla {
    pub const fn new(color: Color) -> Self {
        let Color { r, g, b, a } = color;

        // f32.min and f32.max are not available in const
        // so we have to do it manually
        let max = if r > g {
            if r > b {
                r
            } else {
                b
            }
        } else if g > b {
            g
        } else {
            b
        };
        let min = if r < g {
            if r < b {
                r
            } else {
                b
            }
        } else if g < b {
            g
        } else {
            b
        };

        let l = (max + min) / 2.;

        if max == min {
            return Self { h: 0., s: 0., l, a }; // achromatic
        }
        let d = max - min;
        let s = d / if l > 0.5 { 2. - max - min } else { max + min };
        let h = if r == max {
            (g - b) / d + if g < b { 6.0 } else { 0.0 }
        } else if g == max {
            (b - r) / d + 2.
        } else {
            (r - g) / d + 4.
        } / 6.;

        Self {
            h: h * 360.,
            s,
            l,
            a,
        }
    }

    pub const fn to_color(self) -> Color {
        let Hsla { h, s, l, a } = self;
        if s == 0. {
            return Color::rgbaf(l, l, l, a); // achromatic
        }

        const fn hue2rgb(p: f32, q: f32, t: f32) -> f32 {
            let mut t = t;
            if t < 0.0 {
                t += 1.0;
            }
            if t > 1.0 {
                t -= 1.0;
            }
            if t < 1.0 / 6.0 {
                p + (q - p) * 6.0 * t
            } else if t < 1.0 / 2.0 {
                q
            } else if t < 2.0 / 3.0 {
                p + (q - p) * (2.0 / 3.0 - t) * 6.0
            } else {
                p
            }
        }

        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };
        let p = 2.0 * l - q;
        let r = hue2rgb(p, q, h + 1.0 / 3.0);
        let g = hue2rgb(p, q, h);
        let b = hue2rgb(p, q, h - 1.0 / 3.0);
        Color::rgbaf(r, g, b, a)
    }
}

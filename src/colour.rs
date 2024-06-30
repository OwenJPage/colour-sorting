use {
    crate::{
        circle_degrees::CircleDegrees,
        percentage_f32::PercentageF32,
    },
    std::fmt::{
        Debug,
        Formatter,
    },
};

#[non_exhaustive]
pub enum Colour {
    HSL {
        hue:        CircleDegrees,
        saturation: PercentageF32,
        luminosity: PercentageF32,
    },
    HSV {
        hue:        CircleDegrees,
        saturation: PercentageF32,
        value:      PercentageF32,
    },
    RGB {
        red:   u8,
        green: u8,
        blue:  u8,
    },
    CIELAB {
        l_star: PercentageF32,
        a_star: i8,
        b_star: i8,
    }, // TODO: LAB and other systems
    XYZ {
        x: i8,
        y: i8,
        z: i8,
    },
    LMS {
        // TODO: Figure out the range of LMS values
        l: u8,
        m: u8,
        s: u8,
    },
}

impl Colour {
    #[inline]
    pub fn from_hex(from: u32) -> Self {
        Self::RGB {
            red:   (from >> (2 * u8::BITS)) as u8,
            green: (from >> u8::BITS) as u8,
            blue:  from as u8,
        }
    }

    #[inline]
    pub fn mutate_hsv(&mut self) {
        *self = self.as_hsv();
    }

    fn rgb_to_hsvl<const H: bool, const S: bool, const VL: bool, const VL_IS_L: bool>(
        red: u8,
        green: u8,
        blue: u8,
    ) -> (
        Option<CircleDegrees>,
        Option<PercentageF32>,
        Option<PercentageF32>,
    ) {
        let r_float = (red as f32) / 255f32;
        let g_float = (green as f32) / 255f32;
        let b_float = (blue as f32) / 255f32;

        let max = r_float.max(g_float.max(b_float));
        let min = r_float.min(g_float.min(b_float));

        let h = H.then(|| {
            if max == min {
                CircleDegrees::new_wrapped(0)
            } else {
                let range = max - min;

                let component = match max {
                    m if m == r_float => ((g_float - b_float) / range) % 6.0,
                    m if m == g_float => (b_float - r_float) / range + 2.0,
                    m if m == b_float => (r_float - g_float) / range + 4.0,
                    _ => panic!("Max does not match any RGB component"),
                };

                CircleDegrees::new_wrapped((component * 60.0).round() as i16)
            }
        });

        let s = S.then(|| (max - min) / max);

        let vl = VL.then(|| {
            if VL_IS_L {
                (max + min) / 2.0
            } else {
                max
            }
        });

        (
            h,
            s.map(PercentageF32::new_or_panic),
            vl.map(PercentageF32::new_or_panic),
        )
    }

    fn select_hsv<const H: bool, const S: bool, const V: bool>(
        &self,
    ) -> (
        Option<CircleDegrees>,
        Option<PercentageF32>,
        Option<PercentageF32>,
    ) {
        match self {
            Colour::HSV {
                hue,
                saturation,
                value,
            } => {
                (
                    H.then_some(*hue),
                    S.then_some(*saturation),
                    V.then_some(*value),
                )
            },
            Colour::HSL {
                hue,
                saturation,
                luminosity,
            } => {
                let s_raw = saturation.value();
                let l_raw = luminosity.value();

                let v = const { V || S }.then(|| l_raw + s_raw * l_raw.max(1f32 - l_raw));
                let s = S.then(|| {
                    let v = v.expect("Value was not calculated");

                    if v == 0.0 {
                        0.0
                    } else {
                        2.0 * (1.0 - l_raw / v)
                    }
                });

                (
                    H.then_some(*hue),
                    s.map(PercentageF32::new_or_panic),
                    v.map(PercentageF32::new_or_panic),
                )
            },
            Colour::RGB { red, green, blue } => {
                Self::rgb_to_hsvl::<H, S, V, false>(*red, *green, *blue)
            },
            Colour::CIELAB {
                l_star,
                a_star,
                b_star,
            } => {
                todo!()
            },
        }
    }

    fn select_hsl<const H: bool, const S: bool, const L: bool>(
        &self,
    ) -> (
        Option<CircleDegrees>,
        Option<PercentageF32>,
        Option<PercentageF32>,
    ) {
        match self {
            Colour::HSV {
                hue,
                saturation,
                value,
            } => {
                let s_raw = saturation.value();
                let v_raw = value.value();

                let l = const { L || S }.then(|| v_raw * (1.0 - (s_raw / 2.0)));

                let s = S.then(|| {
                    if v_raw == 0.0 || v_raw == 1.0 {
                        0.0
                    } else {
                        let l_val = l.expect("Luminosity value was not calculated");

                        (v_raw - l_val) / l_val.min(1.0 - l_val)
                    }
                });

                (
                    H.then_some(*hue),
                    s.map(PercentageF32::new_or_panic),
                    l.map(PercentageF32::new_or_panic),
                )
            },
            Colour::HSL {
                hue,
                saturation,
                luminosity,
            } => {
                (
                    H.then_some(*hue),
                    S.then_some(*saturation),
                    L.then_some(*luminosity),
                )
            },
            Colour::RGB { red, green, blue } => {
                Self::rgb_to_hsvl::<H, S, L, true>(*red, *green, *blue)
            },
            Colour::CIELAB {
                l_star,
                a_star,
                b_star,
            } => {
                todo!()
            },
        }
    }

    fn select_rgb<const R: bool, const G: bool, const B: bool>(
        &self,
    ) -> (Option<u8>, Option<u8>, Option<u8>) {
        match self {
            Colour::HSL {
                hue,
                saturation,
                luminosity,
            } => {
                let h_raw = hue.value();
                let s_raw = saturation.value();
                let l_raw = luminosity.value();

                let a = s_raw * f32::min(l_raw, 1.0 - l_raw);

                let f = |n| {
                    let k = (n as f32 + (h_raw as f32 / 30.0)) % 12.0;

                    (l_raw - a * f32::max(-1.0, f32::min(k - 3.0, f32::min(9.0 - k, 1.0)))).round()
                        as u8
                };

                (R.then(|| f(0)), G.then(|| f(8)), B.then(|| f(4)))
            },
            Colour::HSV {
                hue,
                saturation,
                value,
            } => {
                let h_raw = hue.value();
                let s_raw = saturation.value();
                let v_raw = value.value();

                let f = |n| {
                    let k = (n as f32 + (h_raw as f32 / 60.0)) % 6.0;

                    (v_raw - v_raw * s_raw * f32::max(0.0, f32::min(k, f32::min(4.0 - k, 1.0))))
                        .round() as u8
                };

                (R.then(|| f(5)), G.then(|| f(3)), B.then(|| f(1)))
            },
            Colour::RGB { red, green, blue } => {
                (R.then_some(*red), G.then_some(*green), B.then_some(*blue))
            },
            Colour::CIELAB {
                l_star,
                a_star,
                b_star,
            } => {
                todo!()
            },
        }
    }

    pub fn as_hsv(&self) -> Self {
        let (hue, saturation, value) = self.hsv_tuple();
        Self::HSV {
            hue,
            saturation,
            value,
        }
    }

    pub fn as_hsl(&self) -> Self {
        let (hue, saturation, luminosity) = self.hsl_tuple();
        Self::HSL {
            hue,
            saturation,
            luminosity,
        }
    }

    pub fn hsv_tuple(&self) -> (CircleDegrees, PercentageF32, PercentageF32) {
        let (h, s, v) = self.select_hsv::<true, true, true>();

        (
            h.expect("Hue was not returned"),
            s.expect("Saturation was not returned"),
            v.expect("Value was not returned"),
        )
    }

    pub fn hsl_tuple(&self) -> (CircleDegrees, PercentageF32, PercentageF32) {
        let (h, s, l) = self.select_hsl::<true, true, true>();

        (
            h.expect("Hue was not returned"),
            s.expect("Saturation was not returned"),
            l.expect("Luminosity was not returned"),
        )
    }

    pub fn rgb_tuple(&self) -> (u8, u8, u8) {
        let (r, g, b) = self.select_rgb::<true, true, true>();

        (
            r.expect("Red was not calculated"),
            g.expect("Green was not calculated"),
            b.expect("Blue was not calculated"),
        )
    }

    pub fn hue(&self) -> CircleDegrees {
        let (h, _, _) = self.select_hsv::<true, false, false>();
        h.expect("Hue was not calculated")
    }

    pub fn saturation_hsv(&self) -> PercentageF32 {
        let (_, s, _) = self.select_hsv::<false, true, false>();
        s.expect("Saturation was not calculated")
    }

    pub fn value_hsv(&self) -> PercentageF32 {
        let (_, _, v) = self.select_hsv::<false, false, true>();
        v.expect("Value was not calculated")
    }

    pub fn saturation_hsl(&self) -> PercentageF32 {
        let (_, s, _) = self.select_hsl::<false, true, false>();
        s.expect("Saturation was not calculated")
    }

    pub fn luminosity(&self) -> PercentageF32 {
        let (_, _, l) = self.select_hsl::<false, false, true>();
        l.expect("Luminosity was not calculated")
    }

    pub fn red(&self) -> u8 {
        let (r, _, _) = self.select_rgb::<true, false, false>();
        r.expect("Red was not calculated")
    }

    pub fn green(&self) -> u8 {
        let (_, g, _) = self.select_rgb::<false, true, false>();
        g.expect("Green was not calculated")
    }

    pub fn blue(&self) -> u8 {
        let (_, _, b) = self.select_rgb::<false, false, true>();
        b.expect("Blue was not calculated")
    }
}

impl Debug for Colour {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Colour::")?;
        match self {
            Colour::HSL {
                hue,
                saturation,
                luminosity,
            } => {
                f.debug_tuple("HSL")
                    .field(hue)
                    .field(saturation)
                    .field(luminosity)
                    .finish()
            },
            Colour::HSV {
                hue,
                saturation,
                value,
            } => {
                f.debug_tuple("HSV")
                    .field(hue)
                    .field(saturation)
                    .field(value)
                    .finish()
            },
            Colour::RGB { red, green, blue } => {
                f.debug_tuple("RGB")
                    .field(red)
                    .field(green)
                    .field(blue)
                    .finish()
            },
            Colour::CIELAB {
                l_star,
                a_star,
                b_star,
            } => {
                todo!()
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::colour::Colour;

    #[test]
    fn test_from_hex() -> Result<(), String> {
        let colour = Colour::from_hex(0xece5db);

        (colour.red() == 236)
            .then_some(())
            .ok_or(format!("Incorrect red value {}", colour.red()))?;

        (colour.green() == 229)
            .then_some(())
            .ok_or(format!("Incorrect green value {}", colour.green()))?;

        (colour.blue() == 219)
            .then_some(())
            .ok_or(format!("Incorrect blue value {}", colour.blue()))?;

        Ok(())
    }

    #[test]
    fn test_rgb_to_hsv() -> Result<(), String> {
        // RGB(165, 102, 173) -> HSV(293, 41, 68)

        let start = Colour::RGB {
            red:   165,
            green: 102,
            blue:  173,
        };

        let end = start.as_hsv();

        println!("{:?} -> {:?}", start, end);

        (end.hue().value() == 293)
            .then_some(())
            .ok_or(format!("Incorrect hue value {}", end.hue().value()))?;
        (0.405 <= end.saturation_hsv().value() && end.saturation_hsv().value() < 0.415)
            .then_some(())
            .ok_or(format!(
                "Incorrect saturation value {}",
                end.saturation_hsv().value()
            ))?;
        (0.675 <= end.value_hsv().value() && end.value_hsv().value() < 0.685)
            .then_some(())
            .ok_or(format!("Incorrect value value {}", end.value_hsv().value()))?;

        Ok(())
    }

    #[test]
    fn test_all() -> Result<(), String> {
        let start = Colour::RGB {
            red:   36,
            green: 237,
            blue:  73,
        };

        todo!()
    }
}

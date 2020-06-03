//! Display

/// Monochrome pixel
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pixel {
    /// Off
    Off,
    /// On
    On,
}

impl Default for Pixel {
    fn default() -> Self {
        Self::Off
    }
}

impl std::fmt::Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Off => write!(f, "░"),
            Self::On => write!(f, "▓"),
        }
    }
}

/// Result from drawing a sprite.
pub enum DrawResult {
    /// Some pixels were erased while drawing
    PixelsOff,
    /// Pixels were only set to `On` state
    PixelsOn,
}

/// Display with 64 * 32 monochrome pixels
pub struct Display {
    pixels: [Pixel; Display::PIXELS],
}

/// X coordinate of a `Pixel` on the `Display`
pub struct XCoordinate(usize);
/// Y coordinate of a `Pixel` on the `Display`
pub struct YCoordinate(usize);

impl Display {
    /// Horizontal pixel count
    pub const WIDTH: usize = 64;
    /// Vertical pixel count
    pub const HEIGHT: usize = 32;
    ///Total number of pixels
    pub const PIXELS: usize = Self::WIDTH * Self::HEIGHT;

    /// Clears the display by setting all pixels to the `Off` state
    pub fn clear(&mut self) {
        for pixel in self.pixels.iter_mut() {
            *pixel = Pixel::Off;
        }
    }

    /// Draw `sprite` at the given `x` + `y` coordinates
    pub fn draw(&mut self, _sprite: Sprite, _x: XCoordinate, _y: YCoordinate) -> DrawResult {
        todo!();
    }
}

impl Default for Display {
    fn default() -> Self {
        Self {
            pixels: [Pixel::default(); Self::PIXELS],
        }
    }
}

/// Row of 8 pixels in a sprite
#[derive(Debug, PartialEq, Eq)]
pub struct SpriteRow([Pixel; 8]);

impl From<u8> for SpriteRow {
    fn from(bits: u8) -> Self {
        let p0 = if bits & 0b1000_0000 == 0b1000_0000 {
            Pixel::On
        } else {
            Pixel::Off
        };
        let p1 = if bits & 0b0100_0000 == 0b0100_0000 {
            Pixel::On
        } else {
            Pixel::Off
        };
        let p2 = if bits & 0b0010_0000 == 0b0010_0000 {
            Pixel::On
        } else {
            Pixel::Off
        };
        let p3 = if bits & 0b0001_0000 == 0b0001_0000 {
            Pixel::On
        } else {
            Pixel::Off
        };
        let p4 = if bits & 0b0000_1000 == 0b0000_1000 {
            Pixel::On
        } else {
            Pixel::Off
        };
        let p5 = if bits & 0b0000_0100 == 0b0000_0100 {
            Pixel::On
        } else {
            Pixel::Off
        };
        let p6 = if bits & 0b0000_0010 == 0b0000_0010 {
            Pixel::On
        } else {
            Pixel::Off
        };
        let p7 = if bits & 0b0000_0001 == 0b0000_0001 {
            Pixel::On
        } else {
            Pixel::Off
        };

        Self([p0, p1, p2, p3, p4, p5, p6, p7])
    }
}

impl std::fmt::Display for SpriteRow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for pixel in &self.0 {
            write!(f, "{}", pixel)?;
        }
        Ok(())
    }
}

/// Sprite of several rows of pixels
#[derive(Debug, PartialEq, Eq)]
pub struct Sprite {
    rows: Vec<SpriteRow>,
}

impl From<&[u8]> for Sprite {
    fn from(rows: &[u8]) -> Self {
        let mut sprite_rows = Vec::with_capacity(rows.len());
        for row in rows {
            let sprite_row = SpriteRow::from(*row);
            sprite_rows.push(sprite_row);
        }

        Self { rows: sprite_rows }
    }
}

impl std::fmt::Display for Sprite {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in &self.rows {
            writeln!(f, "{}", row)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_clear() {
        let mut display = Display::default();

        display.clear();

        for pixel in display.pixels.iter() {
            assert_eq!(*pixel, Pixel::Off);
        }
    }

    #[test]
    fn spriterow_from_u8() {
        let data = 0b1010_0101;

        let sprite_row: SpriteRow = data.into();

        assert_eq!(
            sprite_row,
            SpriteRow([
                Pixel::On,
                Pixel::Off,
                Pixel::On,
                Pixel::Off,
                Pixel::Off,
                Pixel::On,
                Pixel::Off,
                Pixel::On
            ])
        );
    }

    #[test]
    fn sprite_from_u8_slice() {
        let data = [0b1111_0000, 0b0000_1111];

        let sprite: Sprite = data[..].into();

        assert_eq!(
            sprite,
            Sprite {
                rows: vec![
                    SpriteRow([
                        Pixel::On,
                        Pixel::On,
                        Pixel::On,
                        Pixel::On,
                        Pixel::Off,
                        Pixel::Off,
                        Pixel::Off,
                        Pixel::Off
                    ]),
                    SpriteRow([
                        Pixel::Off,
                        Pixel::Off,
                        Pixel::Off,
                        Pixel::Off,
                        Pixel::On,
                        Pixel::On,
                        Pixel::On,
                        Pixel::On
                    ]),
                ]
            }
        );
    }

    #[test]
    fn sprite_display() {
        let data = [0b11111111, 0b10000000, 0b11111100, 0b10000000, 0b10000000];
        let sprite: Sprite = data[..].into();

        let display = format!("{}", sprite);

        assert_eq!(
            display,
            "▓▓▓▓▓▓▓▓\n\
             ▓░░░░░░░\n\
             ▓▓▓▓▓▓░░\n\
             ▓░░░░░░░\n\
             ▓░░░░░░░\n"
        );
    }
}

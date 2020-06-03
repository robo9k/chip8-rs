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

/// Display with 64 * 32 monochrome pixels
pub struct Display {
    pixels: [Pixel; Display::PIXELS],
}

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
}

impl Default for Display {
    fn default() -> Self {
        Self {
            pixels: [Pixel::default(); Self::PIXELS],
        }
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
}

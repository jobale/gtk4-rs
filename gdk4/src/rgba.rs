// Take a look at the license at the top of the repository in the LICENSE file.

use crate::RGBA;
use glib::{translate::*, IntoGStr};
use std::{fmt, str::FromStr};

#[derive(Debug)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`RGBA`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct RGBABuilder(RGBA);

impl Default for RGBABuilder {
    fn default() -> Self {
        Self(RGBA::WHITE)
    }
}

impl RGBABuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`RGBABuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    pub fn blue(mut self, blue: f32) -> Self {
        self.0.set_blue(blue);
        self
    }

    pub fn green(mut self, green: f32) -> Self {
        self.0.set_green(green);
        self
    }

    pub fn red(mut self, red: f32) -> Self {
        self.0.set_red(red);
        self
    }

    pub fn alpha(mut self, alpha: f32) -> Self {
        self.0.set_alpha(alpha);
        self
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`RGBA`].
    #[must_use = "The RGBA returned by this builder should probably be used"]
    pub fn build(self) -> RGBA {
        self.0
    }
}

impl RGBA {
    #[inline]
    pub const fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        skip_assert_initialized!();
        Self {
            inner: ffi::GdkRGBA {
                red,
                green,
                blue,
                alpha,
            },
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates an owned [`RGBA`] like `self` but with the given red value.
    ///
    /// # Example
    ///
    /// ```
    /// # use gdk4::RGBA;
    ///
    /// let rgba = RGBA::new(1.0, 1.0, 1.0, 1.0);
    /// assert_eq!(rgba.with_red(0.5), RGBA::new(0.5, 1.0, 1.0, 1.0));
    /// ```
    #[inline]
    pub const fn with_red(self, red: f32) -> Self {
        Self {
            inner: ffi::GdkRGBA { red, ..self.inner },
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates an owned [`RGBA`] like `self` but with the given green value.
    ///
    /// # Example
    ///
    /// ```
    /// # use gdk4::RGBA;
    ///
    /// let rgba = RGBA::new(1.0, 1.0, 1.0, 1.0);
    /// assert_eq!(rgba.with_green(0.5), RGBA::new(1.0, 0.5, 1.0, 1.0));
    /// ```
    #[inline]
    pub const fn with_green(self, green: f32) -> Self {
        Self {
            inner: ffi::GdkRGBA {
                green,
                ..self.inner
            },
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates an owned [`RGBA`] like `self` but with the given blue value.
    ///
    /// # Example
    ///
    /// ```
    /// # use gdk4::RGBA;
    ///
    /// let rgba = RGBA::new(1.0, 1.0, 1.0, 1.0);
    /// assert_eq!(rgba.with_blue(0.5), RGBA::new(1.0, 1.0, 0.5, 1.0));
    /// ```
    #[inline]
    pub const fn with_blue(self, blue: f32) -> Self {
        Self {
            inner: ffi::GdkRGBA { blue, ..self.inner },
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates an owned [`RGBA`] like `self` but with the given alpha value.
    ///
    /// # Example
    ///
    /// ```
    /// # use gdk4::RGBA;
    ///
    /// let rgba = RGBA::new(1.0, 1.0, 1.0, 1.0);
    /// assert_eq!(rgba.with_alpha(0.5), RGBA::new(1.0, 1.0, 1.0, 0.5));
    /// ```
    #[inline]
    pub const fn with_alpha(self, alpha: f32) -> Self {
        Self {
            inner: ffi::GdkRGBA {
                alpha,
                ..self.inner
            },
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`RGBA`] objects.
    ///
    /// This method returns an instance of [`RGBABuilder`](crate::builders::RGBABuilder) which can be used to create [`RGBA`] objects.
    pub fn builder() -> RGBABuilder {
        RGBABuilder::default()
    }

    #[inline]
    pub fn red(&self) -> f32 {
        self.inner.red
    }

    #[inline]
    pub fn set_red(&mut self, red: f32) {
        self.inner.red = red;
    }

    #[inline]
    pub fn green(&self) -> f32 {
        self.inner.green
    }

    #[inline]
    pub fn set_green(&mut self, green: f32) {
        self.inner.green = green;
    }

    #[inline]
    pub fn blue(&self) -> f32 {
        self.inner.blue
    }

    #[inline]
    pub fn set_blue(&mut self, blue: f32) {
        self.inner.blue = blue;
    }

    #[inline]
    pub fn alpha(&self) -> f32 {
        self.inner.alpha
    }

    #[inline]
    pub fn set_alpha(&mut self, alpha: f32) {
        self.inner.alpha = alpha;
    }

    #[doc(alias = "gdk_rgba_parse")]
    pub fn parse(s: impl IntoGStr) -> Result<RGBA, glib::error::BoolError> {
        skip_assert_initialized!();
        unsafe {
            s.run_with_gstr(|s| {
                let mut res = RGBA::uninitialized();
                glib::result_from_gboolean!(
                    ffi::gdk_rgba_parse(res.to_glib_none_mut().0, s.as_ptr()),
                    "Can't parse RGBA"
                )
                .map(|_| res)
            })
        }
    }

    pub const BLACK: RGBA = Self::new(0f32, 0f32, 0f32, 1f32);

    pub const BLUE: RGBA = Self::new(0f32, 0f32, 1f32, 1f32);

    pub const GREEN: RGBA = Self::new(0f32, 1f32, 0f32, 1f32);

    pub const RED: RGBA = Self::new(1f32, 0f32, 0f32, 1f32);

    pub const WHITE: RGBA = Self::new(1f32, 1f32, 1f32, 1f32);

    pub const TRANSPARENT: RGBA = Self::new(0f32, 0f32, 0f32, 0f32);
}

impl fmt::Debug for RGBA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("RGBA")
            .field("red", &self.red())
            .field("green", &self.green())
            .field("blue", &self.blue())
            .field("alpha", &self.alpha())
            .finish()
    }
}

impl FromStr for RGBA {
    type Err = glib::BoolError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        skip_assert_initialized!();
        RGBA::parse(s)
    }
}

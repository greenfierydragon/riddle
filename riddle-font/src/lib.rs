#![deny(clippy::all)]

//! Riddle crate for loading font files and rendering text to riddle_image images.
//!
//! Built largely on the back of `rusttype` and its dependencies.
//!
//! # Example
//!
//! ```
//! # use riddle_font::*;
//! # use riddle_common::Color;
//! # fn main() -> Result<(), FontError> {
//! // Load font from TTF file
//! let ttf_bytes = include_bytes!("../../example_assets/Roboto-Regular.ttf");
//! let font = TtFont::load(&ttf_bytes[..])?;
//!
//! // Render the loaded font to a Riddle image
//! let image = font.render_simple("Simple String", 24, Color::BLACK)?;
//! # Ok(())
//! # }
//! ```

mod error;
mod imgfont;
mod ttfont;

pub mod rusttype_ext;

pub use error::*;
pub use imgfont::*;
pub use ttfont::TtFont;

use riddle_common::CommonError;

type Result<R> = std::result::Result<R, FontError>;

pub mod alpha;
pub mod mono;
pub mod color;
pub mod thumbnail;

pub use alpha::generate_alpha_png;
pub use mono::{generate_grayscale_svg, generate_halftone_svg, generate_lineart_svg};
pub use color::generate_color_svg;
pub use thumbnail::generate_thumbnail;

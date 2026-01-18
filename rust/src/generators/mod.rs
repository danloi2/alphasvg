use std::sync::mpsc::Sender;

pub mod alpha;
pub mod mono;
pub mod color;
pub mod thumbnail;
pub mod models;
pub mod ai;

pub use alpha::generate_alpha_png;
pub use mono::{generate_grayscale_svg, generate_halftone_svg, generate_lineart_svg};
pub use color::{generate_logo, generate_illustration};
pub use thumbnail::generate_thumbnail;

#[derive(Clone, PartialEq, Debug)]
pub enum ModelState {
    Unloaded,
    Loading,
    Ready(String),
}

#[derive(Clone, Copy, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub enum ModelType {
    U2Net,
    U2NetP,
    U2NetHumanSeg,
    U2NetClothSeg,
    Silueta,
    IsNetGeneralUse,
    IsNetAnime,
    Sam,
    BiRefNetGeneral,
    BiRefNetGeneralLite,
    BiRefNetPortrait,
    BiRefNetDis,
    BiRefNetHrsod,
    BiRefNetCod,
    BiRefNetMassive,
    BriaRmbg,
}

impl Default for ModelType {
    fn default() -> Self {
        ModelType::U2Net
    }
}

pub enum LogOutput {
    StdOut,
    Channel(Sender<String>),
}

impl LogOutput {
    pub fn send(&self, msg: String) {
        match self {
            LogOutput::StdOut => println!("{}", msg),
            LogOutput::Channel(tx) => { let _ = tx.send(msg); }
        }
    }
}

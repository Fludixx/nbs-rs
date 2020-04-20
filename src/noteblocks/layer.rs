use super::note::Note;
use crate::NbsFormat;
use std::collections::HashMap;

/// A Layer contains an list of notes and some additional information.
#[derive(Debug)]
pub struct Layer {
    /// Name of the layer.
    pub name: String,
    /// Only avabile in the new format version 4.
    pub locked: Option<bool>,
    /// Only avabile in the new format since version 2.
    pub volume: Option<i8>,
    /// Only avabile in the new format since version 2.
    pub stereo: Option<i8>,
    pub notes: HashMap<i16, Note>,
}

impl Layer {
    /// Creates an new empty Layer.
    pub fn new() -> Self {
        Layer {
            name: String::new(),
            locked: None,
            volume: None,
            stereo: None,
            notes: HashMap::new(),
        }
    }

    /// Creates an new Layer with default values for the specified format
    pub fn from_format(format: NbsFormat) -> Self {
        let mut layer = Layer::new();
        layer.name = String::new();
        if format.version() >= 4 {
            layer.locked = Some(false);
        }
        if format.version() >= 2 {
            layer.volume = Some(100);
            layer.stereo = Some(100);
        }
        layer
    }
}

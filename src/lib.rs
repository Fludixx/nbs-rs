//! This crate provides functionality for decoding & encoding (Open)NoteBlockStudio buffers.
//!
//! It supports the original NBS format, aswell as version 1-4 of the unofficial new format introduced in [OpenNoteBlockStudio](https://github.com/HielkeMinecraft/OpenNoteBlockStudio).
//! Documentation on the NBS format can be found at [NoteBlockStudio](https://www.stuffbydavid.com/mcnbs/format) and [OpenNoteBlockStudio](https://hielkeminecraft.github.io/OpenNoteBlockStudio/nbs).
//!
//! ## Example: Editing a NBS file
//!
//! ```rust
//! use nbs::{
//!     noteblocks::{instrument, note::Note},
//!     Nbs,
//! };
//! use std::fs::File;
//!
//! fn main() {
//!     let mut file = File::open("tests/1.nbs").unwrap();
//!     let mut nbs = Nbs::decode(&mut file).unwrap();
//!     nbs.noteblocks.layers[2].name = String::from("Cows"); // Renaming the 3rd layer "Cows".
//!     nbs.noteblocks.layers[2].volume = 25; // Setting its volume to 25%.
//!     // Insert a Note in the 3rd layer at tick 0
//!     nbs.noteblocks.layers[2].notes.insert(
//!         0,
//!         Note::new(instrument::COW_BELL, 33, Some(100), Some(100), Some(0)),
//!     );
//!     // Write the changes to `out1.nbs`.
//!     nbs.encode(&mut File::create("out1.nbs").unwrap()).unwrap();
//! }
//! ```
//! ## Example: Creating a NBS file
//! ```rust
//! use nbs::{
//!     header::Header,
//!     noteblocks::{instrument, instrument::CustomInstruments, layer::Layer, note::Note, NoteBlocks},
//!     Nbs, NbsFormat,
//! };
//! use std::fs::File;
//!
//! fn main() {
//!     let mut file = File::create("out2.nbs").unwrap();
//!     let mut header = Header::new(NbsFormat::OpenNoteBlockStudio(4)); // Create a header.
//!     header.song_name = String::from("test"); // Change the name to `test`.
//!     let mut noteblocks = NoteBlocks::new();
//!     // Create a new Layer.
//!     noteblocks
//!         .layers
//!         .push(Layer::from_format(NbsFormat::OpenNoteBlockStudio(4)));
//!     // Insert 20 notes into the first layer
//!     for i in 0..20 {
//!         noteblocks.layers[0].notes.insert(
//!             i,
//!             Note::new(
//!                 instrument::PIANO,
//!                 (33 + i) as i8,
//!                 Some(100),
//!                 Some(100),
//!                 Some(0),
//!             ),
//!         );
//!     }
//!     let custom_instruments = CustomInstruments::new(); // Create a empty list of custom instruments.
//!     let mut nbs = Nbs::from_componets(header, noteblocks, custom_instruments); // Assamble everything together.
//!     nbs.update(); // Update certian fields in the header to match the rest of the file.
//!     nbs.encode(&mut file); // save!
//! }
//! ```

use error::NbsError;
use header::Header;
use io::{ReadStringExt, WriteStringExt};
use noteblocks::{instrument::CustomInstruments, NoteBlocks};
use std::time::Duration;

pub mod error;
pub mod header;
pub mod io;
pub mod noteblocks;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum NbsFormat {
    NoteBlockStudio,
    OpenNoteBlockStudio(i8),
}
impl NbsFormat {
    pub fn is_new(&self) -> bool {
        match self {
            NbsFormat::NoteBlockStudio => false,
            NbsFormat::OpenNoteBlockStudio(_) => true,
        }
    }
    pub fn version(&self) -> i8 {
        match self {
            NbsFormat::NoteBlockStudio => 0,
            &NbsFormat::OpenNoteBlockStudio(v) => v,
        }
    }
}

pub struct Nbs {
    pub header: Header,
    pub noteblocks: NoteBlocks,
    pub custom_instruments: CustomInstruments,
}

impl Nbs {
    pub fn from_componets(
        header: Header,
        noteblocks: NoteBlocks,
        custom_instruments: CustomInstruments,
    ) -> Self {
        Nbs {
            header,
            noteblocks,
            custom_instruments,
        }
    }

    /// Decode a NBS buffer.
    pub fn decode<R>(mut reader: &mut R) -> Result<Nbs, NbsError>
    where
        R: ReadStringExt,
    {
        let header = Header::decode(&mut reader)?;
        let noteblocks = NoteBlocks::decode(&mut reader, &header)?;
        let custom_instruments = CustomInstruments::decode(&mut reader, &header)?;
        Ok(Nbs {
            header,
            noteblocks,
            custom_instruments,
        })
    }

    /// This method updates some parts of the Header to match the rest of the file
    pub fn update(&mut self) {
        if self.format().version() >= 3 {
            self.header.song_length = Some(self.noteblocks.calculate_length());
        } else if self.format().version() == 0 {
            self.header.old_song_length = self.noteblocks.calculate_length();
        }
        if self.format().version() > 0 {
            self.header.version_number = Some(self.format().version());
        }
        self.header.layer_count = self.noteblocks.layers.len() as i16;
    }

    /// Enocde a NBS buffer,
    pub fn encode<W>(&self, mut writer: &mut W) -> Result<(), NbsError>
    where
        W: WriteStringExt,
    {
        self.header.encode(self.format(), &mut writer)?;
        self.noteblocks.encode(self.format(), &mut writer)?;
        self.custom_instruments.encode(&mut writer)?;
        Ok(())
    }

    /// Returns the NBS format for this
    pub fn format(&self) -> NbsFormat {
        return self.header.format;
    }

    /// Returns the song ticks.
    pub fn song_ticks(&self) -> i16 {
        self.noteblocks.calculate_length()
    }

    /// Returns the song duration.
    pub fn song_length(&self) -> Duration {
        Duration::from_secs_f32(self.song_ticks() as f32 / (self.header.song_tempo as f32 / 100.0))
    }
}

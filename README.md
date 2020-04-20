This crate provides functionality for decoding & encoding (Open)NoteBlockStudio buffers.
It supports the original NBS format, aswell as version 1-4 of the unofficial new format introduced in [OpenNoteBlockStudio](https://github.com/HielkeMinecraft/OpenNoteBlockStudio).
Documentation on the NBS format can be found at [NoteBlockStudio](https://www.stuffbydavid.com/mcnbs/format) and [OpenNoteBlockStudio](https://hielkeminecraft.github.io/OpenNoteBlockStudio/nbs).
## Example: Editing a NBS file
```rust
use nbs::{
   noteblocks::{instrument::Instrument, note::Note},
   Nbs,
};
use std::fs::File;
fn main() {
   let mut file = File::open("tests/1.nbs").unwrap();
   let mut nbs = Nbs::decode(&mut file).unwrap();
   nbs.noteblocks.layers[2].name = String::from("Cows"); // Renaming the 3rd layer "Cows".
   nbs.noteblocks.layers[2].volume = Some(25); // Setting its volume to 25%.
   // Insert a Note in the 3rd layer at tick 0
   nbs.noteblocks.layers[2].notes.insert(
       0,
       Note::new(Instrument::CowBell, 33, Some(100), Some(100), Some(0)),
   );
   // Write the changes to `out1.nbs`.
   nbs.encode(&mut File::create("out1.nbs").unwrap()).unwrap();
}
```
## Example: Creating a NBS file
```rust
use nbs::{
    header::Header,
    noteblocks::{
        instrument::CustomInstruments, instrument::Instrument, layer::Layer, note::Note, NoteBlocks,
    },
    Nbs, NbsFormat,
};
use std::fs::File;
fn main() {
   let mut file = File::create("out2.nbs").unwrap();
   let mut header = Header::new(NbsFormat::OpenNoteBlockStudio(4)); // Create a header.
   header.song_name = String::from("test"); // Change the name to `test`.
   let mut noteblocks = NoteBlocks::new();
   // Create a new Layer.
   noteblocks.layers.push(Layer::from_format(NbsFormat::OpenNoteBlockStudio(4)));
   for i in 0..20 { // Insert 20 notes into the first layer
       noteblocks.layers[0].notes.insert(
           i, Note::new(Instrument::Piano, (33 + i) as i8, Some(100), Some(100), Some(0))
       );
   }
   // Create a empty list of custom instruments.
   let custom_instruments = CustomInstruments::new();
   // Assamble everything together.
   let mut nbs = Nbs::from_componets(header, noteblocks, custom_instruments);
   nbs.update(); // Update certian fields in the header to match the rest of the file.
   nbs.encode(&mut file); // save!
}
```
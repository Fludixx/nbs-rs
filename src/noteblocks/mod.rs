use crate::{header::Header, NbsError, NbsFormat};
use byteorder::LittleEndian;
use instrument::Instrument;
use layer::Layer;
use note::Note;

pub mod instrument;
pub mod layer;
pub mod note;

#[derive(Debug)]
pub struct NoteBlocks {
    /// Layers of the File.
    pub layers: Vec<Layer>,
}

impl NoteBlocks {
    pub fn new() -> Self {
        NoteBlocks { layers: Vec::new() }
    }
}

impl NoteBlocks {
    pub fn calculate_length(&self) -> i16 {
        let mut length: i16 = 0;
        for layer in &self.layers {
            if layer.notes.len() > 0 {
                let last_note = layer.notes.iter().max_by(|x, y| x.0.cmp(&y.0)).unwrap();
                if last_note.0 > &length {
                    length = *last_note.0;
                }
            }
        }
        length
    }

    pub fn decode<R>(reader: &mut R, header: &Header) -> Result<NoteBlocks, NbsError>
    where
        R: crate::ReadStringExt,
    {
        let mut noteblocks = NoteBlocks::new();
        // If the layer count differs from the encoded layers, NoteBlockStudio crashes.
        for layer_index in 0..header.layer_count {
            let layer = Layer::from_format(header.format);
            noteblocks.layers.insert(layer_index as usize, layer);
        }

        let mut tick: i16 = -1;
        loop {
            let mut jumps = reader.read_i16::<LittleEndian>()?;
            if jumps == 0 {
                break;
            }
            tick += jumps;
            let mut layer: i16 = -1;
            loop {
                jumps = reader.read_i16::<LittleEndian>()?;
                if jumps == 0 {
                    break;
                }
                layer += jumps;
                let instrument = reader.read_i8()?;

                let instrument = if instrument >= header.vannila_instrument_count()? {
                    Instrument::Custom(instrument)
                } else {
                    Instrument::Vanilla(instrument)
                };
                let key = reader.read_i8()?;
                let velocity = if header.format.version() >= 4 {
                    Some(reader.read_i8()?)
                } else {
                    None
                };
                let panning = if header.format.version() >= 4 {
                    Some(reader.read_i8()?)
                } else {
                    None
                };
                let pitch = if header.format.version() >= 4 {
                    Some(reader.read_i16::<LittleEndian>()?)
                } else {
                    None
                };
                noteblocks
                    .layers
                    .get_mut(layer as usize)
                    .unwrap()
                    .notes
                    .insert(
                        tick,
                        Note {
                            instrument,
                            key,
                            velocity,
                            panning,
                            pitch,
                        },
                    );
            }
        }
        for layer_index in 0..noteblocks.layers.len() {
            let mut layer = noteblocks.layers.get_mut(layer_index).unwrap();
            (*layer).name = reader.read_string()?;
            if header.format.version() >= 4 {
                (*layer).locked = Some(reader.read_i8()? == 1);
            }
            (*layer).volume = reader.read_i8()?;
            if header.format.version() >= 2 {
                (*layer).stereo = Some(reader.read_i8()?);
            }
        }
        Ok(noteblocks)
    }
    pub fn encode<W>(&self, format: NbsFormat, writer: &mut W) -> Result<(), NbsError>
    where
        W: crate::WriteStringExt,
    {
        let mut h_cursor: i16 = -1;
        for note_index in 0..=self.calculate_length() {
            let mut v_cursor: i16 = -1;
            let h_jumps = note_index - h_cursor;
            let mut has_jumped_h = false;
            for (layer_index, layer) in self.layers.iter().enumerate() {
                if layer.notes.contains_key(&note_index) {
                    let v_jumps = (layer_index as i16) - v_cursor;
                    let note = layer.notes.get(&note_index).unwrap();
                    if !has_jumped_h {
                        writer.write_i16::<LittleEndian>(h_jumps)?;
                        has_jumped_h = true;
                        h_cursor += h_jumps;
                    }
                    writer.write_i16::<LittleEndian>(v_jumps)?;
                    v_cursor += v_jumps;
                    writer.write_i8(note.instrument.into())?;
                    writer.write_i8(note.key)?;
                    if format.version() >= 4 {
                        writer.write_i8(note.velocity.ok_or(NbsError::InvalidFormat)?)?;
                        writer.write_i8(note.panning.ok_or(NbsError::InvalidFormat)?)?;
                        writer.write_i16::<LittleEndian>(
                            note.pitch.ok_or(NbsError::InvalidFormat)?,
                        )?;
                    }
                }
            }
            if has_jumped_h {
                // If this row actually had notes in it, that means we jumped at least 1 time, we indicate that its finished.
                writer.write_i16::<LittleEndian>(0)?;
            }
        }
        writer.write_i16::<LittleEndian>(0)?;
        for layer_index in 0..self.layers.len() {
            let layer = self.layers.get(layer_index).unwrap();
            writer.write_string(&layer.name)?;
            if format.version() >= 4 {
                writer.write_i8(if layer.locked.ok_or(NbsError::InvalidFormat)? {
                    1
                } else {
                    0
                })?;
            }
            writer.write_i8(layer.volume)?;
            if format.version() >= 2 {
                writer.write_i8(layer.stereo.ok_or(NbsError::InvalidFormat)?)?;
            }
        }
        Ok(())
    }
}

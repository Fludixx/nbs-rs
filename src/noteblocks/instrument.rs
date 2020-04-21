use crate::{header::Header, NbsError};

pub const PIANO: Instrument = Instrument::Vanilla(0);
pub const DOUBLE_BASS: Instrument = Instrument::Vanilla(1);
pub const BASS_DRUM: Instrument = Instrument::Vanilla(2);
pub const SNARE_DRUM: Instrument = Instrument::Vanilla(3);
pub const CLICK: Instrument = Instrument::Vanilla(4);
pub const GUITAR: Instrument = Instrument::Vanilla(5);
pub const FLUTE: Instrument = Instrument::Vanilla(6);
pub const BELL: Instrument = Instrument::Vanilla(7);
pub const CHIME: Instrument = Instrument::Vanilla(8);
pub const XYLOPHONE: Instrument = Instrument::Vanilla(9);
pub const IRON_XYLOPHONE: Instrument = Instrument::Vanilla(10);
pub const COW_BELL: Instrument = Instrument::Vanilla(11);
pub const DIDGERIDOO: Instrument = Instrument::Vanilla(12);
pub const BIT: Instrument = Instrument::Vanilla(13);
pub const BANJO: Instrument = Instrument::Vanilla(14);
pub const PLING: Instrument = Instrument::Vanilla(15);

#[derive(Debug, Clone, Copy)]
pub enum Instrument {
    Vanilla(i8),
    Custom(i8),
}

impl Instrument {
    pub fn is_custom(&self) -> bool {
        match self {
            Instrument::Custom(_) => true,
            _ => false,
        }
    }
}

impl Into<i8> for Instrument {
    fn into(self) -> i8 {
        match self {
            Instrument::Custom(id) | Instrument::Vanilla(id) => id,
        }
    }
}

pub struct CustomInstruments {
    instruments: Vec<CustomInstrumentInfo>,
}

impl CustomInstruments {
    pub fn new() -> Self {
        CustomInstruments {
            instruments: Vec::new(),
        }
    }

    pub fn decode<R>(reader: &mut R, header: &Header) -> Result<CustomInstruments, NbsError>
    where
        R: crate::ReadStringExt,
    {
        let instrument_count = reader.read_i8()?;
        let mut custom_instruments = CustomInstruments {
            instruments: Vec::with_capacity(instrument_count as usize),
        };
        for id in 0..instrument_count {
            // We don't want to overlap with vannila instruments.
            let instrument = Instrument::Custom(id + header.vannila_instrument_count()?);
            let name = reader.read_string()?;
            let file_name = reader.read_string()?;
            let pitch = reader.read_i8()?;
            let press_key = reader.read_i8()? == 1;
            custom_instruments.instruments.push(CustomInstrumentInfo {
                instrument,
                name,
                file_name,
                pitch,
                press_key,
            })
        }
        Ok(custom_instruments)
    }

    pub fn encode<W>(&self, writer: &mut W) -> Result<(), NbsError>
    where
        W: crate::WriteStringExt,
    {
        writer.write_i8(self.instruments.len() as i8)?;
        for instrument in &self.instruments {
            writer.write_string(&instrument.name)?;
            writer.write_string(&instrument.file_name)?;
            writer.write_i8(instrument.pitch)?;
            writer.write_i8(if instrument.press_key { 1 } else { 0 })?;
        }
        Ok(())
    }
}

pub struct CustomInstrumentInfo {
    pub instrument: Instrument,
    pub name: String,
    pub file_name: String,
    pub pitch: i8,
    pub press_key: bool,
}

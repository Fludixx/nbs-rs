use crate::NbsError;

#[derive(Debug, Clone, Copy)]
pub enum Instrument {
    Piano,
    DoubleBass,
    BassDrum,
    SnareDrum,
    Click,
    Guitar,
    Flute,
    Bell,
    Chime,
    Xylophone,
    IronXylophone,
    CowBell,
    Didgeridoo,
    Bit,
    Banjo,
    Pling,
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
            Instrument::Piano => 0,
            Instrument::DoubleBass => 1,
            Instrument::BassDrum => 2,
            Instrument::SnareDrum => 3,
            Instrument::Click => 4,
            Instrument::Guitar => 5,
            Instrument::Flute => 6,
            Instrument::Bell => 7,
            Instrument::Chime => 8,
            Instrument::Xylophone => 9,
            Instrument::IronXylophone => 10,
            Instrument::CowBell => 11,
            Instrument::Didgeridoo => 12,
            Instrument::Bit => 13,
            Instrument::Banjo => 14,
            Instrument::Pling => 15,
            Instrument::Custom(id) => id,
        }
    }
}

impl From<i8> for Instrument {
    fn from(id: i8) -> Self {
        match id {
            0 => Instrument::Piano,
            1 => Instrument::DoubleBass,
            2 => Instrument::BassDrum,
            3 => Instrument::SnareDrum,
            4 => Instrument::Click,
            5 => Instrument::Guitar,
            6 => Instrument::Flute,
            7 => Instrument::Bell,
            8 => Instrument::Chime,
            9 => Instrument::Xylophone,
            10 => Instrument::IronXylophone,
            11 => Instrument::CowBell,
            12 => Instrument::Didgeridoo,
            13 => Instrument::Bit,
            14 => Instrument::Banjo,
            15 => Instrument::Pling,
            _ => Instrument::Custom(id),
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

    pub fn decode<R>(reader: &mut R) -> Result<CustomInstruments, NbsError>
    where
        R: crate::ReadStringExt,
    {
        let instrument_count = reader.read_i8()?;
        let mut custom_instruments = CustomInstruments {
            instruments: Vec::with_capacity(instrument_count as usize),
        };
        for id in 0..instrument_count {
            // we don't want to overlap with vannila instruments, so we add +16
            let instrument = Instrument::Custom(id + 16);
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

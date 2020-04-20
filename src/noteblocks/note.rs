use super::instrument::Instrument;
/// A Note is a Noteblock
#[derive(Debug)]
pub struct Note {
    /// The instrument of the note block.
    /// This is 0-15, or higher if the song uses custom instruments.
    pub instrument: Instrument,
    /// The key of the note block, from 0-87, where 0 is A0 and 87 is C8.
    /// 33-57 is within the 2-octave limit.
    pub key: i8,
    /// The velocity/volume of the note block, from 0% to 100%.
    /// Only avabile in the new format version 4.
    pub velocity: Option<i8>,
    /// The stereo position of the note block, from 0-200.
    /// 100 is center panning.
    /// Only avabile in the new format version 4.
    pub panning: Option<i8>,
    /// The fine pitch of the note block in cents.
    /// The max in Note Block Studio is limited to -1200 and +1200.
    /// 0 is no fine-tuning.
    /// ±100 cents is a single semitone difference.
    /// Only avabile in the new format version 4.
    pub pitch: Option<i16>,
}

impl Note {
    pub fn new(
        instrument: Instrument,
        key: i8,
        velocity: Option<i8>,
        panning: Option<i8>,
        pitch: Option<i16>,
    ) -> Self {
        Note {
            instrument,
            key,
            velocity,
            panning,
            pitch,
        }
    }
}

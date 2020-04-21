use crate::{NbsError, NbsFormat};
use byteorder::LittleEndian;
use std::time::Duration;

/// The header contains information about the file
#[derive(Debug)]
pub struct Header {
    /// The first 2 bytes are always zero in the new fromat.
    /// In the old NBS format, this used to be song length, which can never be zero.
    pub(crate) old_song_length: i16,
    /// The version of the new NBS format.
    /// Only avabile in the new format.
    pub(crate) version_number: Option<i8>,
    /// Amount of default instruments when the song was saved.
    /// This is needed to determine at what index custom instruments start.
    /// Only avabile in the new format
    pub vannila_instrument_count: Option<i8>,
    /// The length of the song, measured in ticks.
    /// Divide this by the tempo to get the length of the song in seconds.
    /// Only avabile in the new format starting from version 3.
    pub(crate) song_length: Option<i16>,
    /// The last layer with at least one note block in it, or the last layer that has had its name, volume or stereo changed.
    pub layer_count: i16,
    /// The name of the song.
    pub song_name: String,
    /// The author of the song.
    pub song_author: String,
    /// The original author of the song.
    pub original_song_author: String,
    /// The description of the song.
    pub song_description: String,
    /// The tempo of the song multiplied by 100.
    pub song_tempo: i16,
    /// Whether auto-saving has been enabled.
    /// As of NBS version 4 this value is still saved to the file, but no longer used in the program.
    pub auto_saving: bool,
    /// The amount of minutes between each auto-save (if it has been enabled) (1-60).
    /// As of NBS version 4 this value is still saved to the file, but no longer used in the program.
    pub auto_saving_duration: i8,
    /// The time signature of the song.
    /// If this is 3, then the signature is 3/4. Default is 4. This value ranges from 2-8.
    pub time_signature: i8,
    /// Amount of minutes spent on the project.
    pub minutes_spent: i32,
    /// Amount of times the user has left-clicked.
    pub left_clicks: i32,
    /// Amount of times the user has right-clicked.
    pub right_clicks: i32,
    /// Amount of times the user has added a note block.
    pub noteblocks_added: i32,
    /// The amount of times the user have removed a note block.
    pub noteblocks_removed: i32,
    /// If the song has been imported from a .mid or .schematic file, that file name is stored here (only the name of the file, not the path).
    pub imported_file_name: String,
    /// Whether looping is on or off.
    /// Only avabile in the new format.
    pub is_loop: Option<bool>,
    /// 0 = infinite. Other values mean the amount of times the song loops.
    /// Only avabile in the new format.
    pub max_loop_count: Option<i8>,
    /// Determines which part of the song (in ticks) it loops back to.
    /// Only avabile in the new format.
    pub loop_start_tick: Option<i16>,
    /// Not part of the Header.
    pub format: NbsFormat,
}

impl Header {
    pub fn new(format: NbsFormat) -> Self {
        Header {
            old_song_length: 0,
            version_number: Some(format.version()),
            vannila_instrument_count: Some(16),
            song_length: Some(0),
            layer_count: 0,
            song_name: String::new(),
            song_author: String::new(),
            original_song_author: String::new(),
            song_description: String::new(),
            song_tempo: 1000,
            auto_saving: false,
            auto_saving_duration: 0,
            time_signature: 4,
            minutes_spent: 0,
            left_clicks: 0,
            right_clicks: 0,
            noteblocks_added: 0,
            noteblocks_removed: 0,
            imported_file_name: String::new(),
            is_loop: Some(false),
            max_loop_count: Some(0),
            loop_start_tick: Some(0),
            format: format,
        }
    }

    pub fn decode<R>(reader: &mut R) -> Result<Self, NbsError>
    where
        R: crate::ReadStringExt,
    {
        let old_song_length = reader.read_i16::<LittleEndian>()?;
        let version = if old_song_length != 0 {
            NbsFormat::NoteBlockStudio
        } else {
            NbsFormat::OpenNoteBlockStudio(reader.read_i8()?)
        };
        let version_number = match version {
            NbsFormat::NoteBlockStudio => None,
            NbsFormat::OpenNoteBlockStudio(v) => Some(v),
        };
        let vannila_instrument_count = if version.is_new() {
            Some(reader.read_i8()?)
        } else {
            None
        };
        let song_length = if version.is_new() {
            Some(reader.read_i16::<LittleEndian>()?)
        } else {
            None
        };
        let layer_count = reader.read_i16::<LittleEndian>()?;
        let song_name = reader.read_string()?;
        let song_author = reader.read_string()?;
        let original_song_author = reader.read_string()?;
        let song_description = reader.read_string()?;
        let song_tempo = reader.read_i16::<LittleEndian>()?;
        let auto_saving = if reader.read_i8()? == 1 { true } else { false };
        let auto_saving_duration = reader.read_i8()?;
        let time_signature = reader.read_i8()?;
        let minutes_spent = reader.read_i32::<LittleEndian>()?;
        let left_clicks = reader.read_i32::<LittleEndian>()?;
        let right_clicks = reader.read_i32::<LittleEndian>()?;
        let noteblocks_added = reader.read_i32::<LittleEndian>()?;
        let noteblocks_removed = reader.read_i32::<LittleEndian>()?;
        let imported_file_name = reader.read_string()?;
        let is_loop = if version.is_new() {
            Some(if reader.read_i8()? == 1 { true } else { false })
        } else {
            None
        };
        let max_loop_count = if version.is_new() {
            Some(reader.read_i8()?)
        } else {
            None
        };
        let loop_start_tick = if version.is_new() {
            Some(reader.read_i16::<LittleEndian>()?)
        } else {
            None
        };
        Ok(Header {
            old_song_length,
            version_number,
            vannila_instrument_count,
            song_length,
            layer_count,
            song_name,
            song_author,
            original_song_author,
            song_description,
            song_tempo,
            auto_saving,
            auto_saving_duration,
            time_signature,
            minutes_spent,
            left_clicks,
            right_clicks,
            noteblocks_added,
            noteblocks_removed,
            imported_file_name,
            is_loop,
            max_loop_count,
            loop_start_tick,
            format: version,
        })
    }

    pub fn encode<W>(&self, format: NbsFormat, writer: &mut W) -> Result<(), NbsError>
    where
        W: crate::WriteStringExt,
    {
        writer.write_i16::<LittleEndian>(self.old_song_length)?;
        if format.version() > 0 {
            writer.write_i8(self.version_number.ok_or(NbsError::InvalidFormat)?)?;
            writer.write_i8(
                self.vannila_instrument_count
                    .ok_or(NbsError::InvalidFormat)?,
            )?;
        }
        if format.version() >= 3 {
            writer.write_i16::<LittleEndian>(self.song_length.ok_or(NbsError::InvalidFormat)?)?;
        }
        writer.write_i16::<LittleEndian>(self.layer_count)?;
        writer.write_string(&self.song_name)?;
        writer.write_string(&self.song_author)?;
        writer.write_string(&self.original_song_author)?;
        writer.write_string(&self.song_description)?;
        writer.write_i16::<LittleEndian>(self.song_tempo)?;
        writer.write_i8(if self.auto_saving { 1 } else { 0 })?;
        writer.write_i8(self.auto_saving_duration)?;
        writer.write_i8(self.time_signature)?;
        writer.write_i32::<LittleEndian>(self.minutes_spent)?;
        writer.write_i32::<LittleEndian>(self.left_clicks)?;
        writer.write_i32::<LittleEndian>(self.right_clicks)?;
        writer.write_i32::<LittleEndian>(self.noteblocks_added)?;
        writer.write_i32::<LittleEndian>(self.noteblocks_removed)?;
        writer.write_string(&self.imported_file_name)?;
        if format.version() > 0 {
            writer.write_i8(if self.is_loop.ok_or(NbsError::InvalidFormat)? {
                1
            } else {
                0
            })?;
            writer.write_i8(self.max_loop_count.ok_or(NbsError::InvalidFormat)?)?;
            writer
                .write_i16::<LittleEndian>(self.loop_start_tick.ok_or(NbsError::InvalidFormat)?)?;
        }

        Ok(())
    }

    pub fn vannila_instrument_count(&self) -> Result<i8, NbsError> {
        Ok(match self.format {
            NbsFormat::NoteBlockStudio => 10,
            NbsFormat::OpenNoteBlockStudio(_) => self
                .vannila_instrument_count
                .ok_or(NbsError::InvalidFormat)?,
        })
    }

    /// Returns the song ticks.
    /// This method will only return valid results for old versions and version 3 and 4 of the new version.
    pub fn song_ticks(&self) -> Result<Option<i16>, NbsError> {
        Ok(match self.format {
            NbsFormat::NoteBlockStudio => Some(self.old_song_length),
            NbsFormat::OpenNoteBlockStudio(v) => {
                if v >= 3 {
                    Some(self.song_length.ok_or(NbsError::InvalidFormat)?)
                } else {
                    None
                }
            }
        })
    }

    /// Returns the song Duration.
    /// This method will only return valid results for old versions and version 3 and 4 of the new version.
    pub fn song_length(&self) -> Result<Option<Duration>, NbsError> {
        let song_ticks = self.song_ticks()?;
        if song_ticks.is_none() {
            return Ok(None);
        }
        Ok(Some(Duration::from_secs_f32(
            song_ticks.unwrap() as f32 / (self.song_tempo as f32 / 100.0),
        )))
    }
}

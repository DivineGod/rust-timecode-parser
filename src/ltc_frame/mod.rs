use crate::ltc_frame::ltc_frame_data::LtcFrameData;
use core::fmt::{Debug, Display, Formatter};

pub(crate) mod ltc_frame_data;

/// Represents 80 bits that represent a ltc-tc-frame
/// Contains functions to push bits received by an audio signal and read it's value as well as functions to write bits to the audio
pub(crate) struct LtcFrame {
    ///Contains the data of the old-frame, if the frame is complete
    data: LtcFrameData,
    /// Tells how many samples it took to get a whole tc-frame without sync-word
    frame_data_sample_count: usize,
}

impl LtcFrame {}

#[cfg(test)]
impl PartialEq<Self> for LtcFrame {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

///Implementations that are used to decode and encode timecode
impl LtcFrame {
    const LTC_SYNC_WORD: u16 = 0b_0011_1111_1111_1101;

    /// Invalidates the current status of the ltc-frame
    pub(crate) fn invalidate(&mut self) {
        self.data.invalidate();
    }
}

#[cfg(feature = "debug")]
impl Debug for LtcFrame {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "data: {:?}", self.data)
    }
}

#[cfg(feature = "debug")]
impl Display for LtcFrame {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "data: {}", self.data)
    }
}

#[cfg(feature = "decode_ltc")]
impl LtcFrame {
    ///Constructor that is used when reading ltc stream from audio
    pub(crate) fn new_empty() -> Self {
        Self {
            data: LtcFrameData::new_empty(),
            frame_data_sample_count: 0,
        }
    }
    ///When a new audio bit is received, this function will shift all received data and add it to the end. Once the sync_word matches, the data is a valid frame
    pub(crate) fn shift_bit(&mut self, bit: bool) {
        self.data.shift_bit(bit);
    }
    ///Tells if all data is received by the audio stream after the sync-word
    pub(crate) fn sync_word_valid(&self) -> bool {
        self.data.get_count() == 0 && self.data.get_sync_word() == Self::LTC_SYNC_WORD
    }
    ///Used to count how many samples a timecode-frame has needed to complete do determine FramesPerSecond of LTC
    pub(crate) fn sample_received(&mut self) {
        if self.data.next_bit_is_start_of_frame() {
            self.frame_data_sample_count = 0;
            self.invalidate();
        } else {
            self.frame_data_sample_count += 1;
        }
    }

    ///Returns the data read from audio decoding only if all data has been received after the sync-word
    /// It may be more efficient to first check if data_valid() returns true due to less memory allocation in ram
    pub(crate) fn get_data(&mut self) -> Option<(LtcFrameData, usize)> {
        if self.sync_word_valid() {
            Some((self.data.clone(), self.frame_data_sample_count))
        } else {
            None
        }
    }
}

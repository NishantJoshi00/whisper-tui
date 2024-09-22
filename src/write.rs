use anyhow::Result;
use std::fmt;
use whisper_rs::{WhisperContext, WhisperContextParameters};

pub struct Writer {
    context: WhisperContext,
}

#[derive(Clone)]
pub struct Text {
    pub text: String,
    pub start: i64,
    pub stop: i64,
}

impl Writer {
    pub fn new(model: &str) -> Result<Self> {
        let context = WhisperContext::new_with_params(model, WhisperContextParameters::default())?;

        Ok(Self { context })
    }

    pub fn generate_text(&self, data: &[f32]) -> Result<Vec<Text>> {
        let mut state = self.context.create_state()?;
        let params =
            whisper_rs::FullParams::new(whisper_rs::SamplingStrategy::Greedy { best_of: 1 });

        state.full(params, data)?;

        let num_seg = state.full_n_segments()?;
        let mut text = Vec::new();

        for i in 0..num_seg {
            let segment = state.full_get_segment_text(i)?;
            let start = state.full_get_segment_t0(i)?;
            let stop = state.full_get_segment_t1(i)?;

            text.push(Text {
                text: segment,
                start,
                stop,
            });
        }

        Ok(text)
    }
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{} - {}]: {}", self.start, self.stop, self.text)
    }
}

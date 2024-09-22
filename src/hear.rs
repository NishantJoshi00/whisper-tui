use std::sync::Mutex;

use anyhow::{anyhow, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::Arc;

pub struct Hearer {
    stream: cpal::Stream,
    state: State,
    buffer: Buffer,
}

enum State {
    Recording { started_at: time::PrimitiveDateTime },
    Stopped,
}

type Buffer = Arc<Mutex<Vec<f32>>>;

impl Hearer {
    pub fn new() -> Result<Self> {
        let buffer: Buffer = Arc::new(Mutex::new(Vec::new()));

        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or_else(|| anyhow!("Failed to get default input device"))?;

        let err_fn = move |err| {
            eprintln!("an error occurred on stream: {}", err);
        };

        let config = cpal::StreamConfig {
            channels: 1,
            sample_rate: cpal::SampleRate(16_000),
            buffer_size: cpal::BufferSize::Fixed(1024),
        };

        let shared_data = Arc::clone(&buffer);

        let stream = device.build_input_stream(
            &config,
            move |data, _: &_| write_input_data(data, shared_data.clone()),
            err_fn,
            None,
        )?;

        Ok(Self {
            stream,
            buffer,
            state: State::Stopped,
        })
    }

    pub fn start(&mut self) -> Result<()> {
        self.stream.play()?;
        let now_odt = time::OffsetDateTime::now_utc();
        let now_pdt = time::PrimitiveDateTime::new(now_odt.date(), now_odt.time());
        self.state = State::Recording {
            started_at: now_pdt,
        };

        Ok(())
    }

    pub fn stop<F, O>(&mut self, callback: F) -> Result<O>
    where
        F: Fn(&[f32], time::PrimitiveDateTime) -> Result<O>,
    {
        let timestamp = self.stop_without_callback()?;

        let channel = self.buffer.lock().unwrap();

        callback(&channel, timestamp)
    }

    pub fn stop_without_callback(&self) -> Result<time::PrimitiveDateTime> {
        self.stream.pause()?;
        match self.state {
            State::Recording { started_at } => Ok(started_at),
            State::Stopped => anyhow::bail!("Stream is already stopped"),
        }
    }
}

fn write_input_data(input: &[f32], channel: Buffer) {
    let mut channel = channel.lock().unwrap();
    channel.extend_from_slice(input);
}

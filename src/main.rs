// Modules
mod decode;
mod math;
mod process;

// Imports
use process::process_signal;
use tinyaudio::prelude::*;

fn main() -> eyre::Result<()> {
    let filepath = "./samples/4.mp3";
    let (sample_rate, raw) = decode::decode_file(filepath)?;
    let (left, right, delay) = process_signal(raw, sample_rate)?;

    let params = OutputDeviceParameters {
        channels_count: 2,
        sample_rate: sample_rate as usize,
        channel_sample_count: 4410,
    };

    let _device = run_output_device(params, {
        let mut idx: usize = 0;
        move |data| {
            for samples in data.chunks_mut(params.channels_count) {
                let lidx = idx as i32 - delay[idx];
                let l = if lidx < 0 {
                    0.0_f32
                } else {
                    left[lidx as usize]
                };

                samples[0] = l;
                samples[1] = right[idx];
                idx += 1;
            }
        }
    })
    .unwrap();

    std::thread::sleep(std::time::Duration::from_secs(500));

    Ok(())
}

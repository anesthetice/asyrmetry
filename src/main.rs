// Modules
mod decode;
mod process;

use process::process_signal;
// Imports
use tinyaudio::prelude::*;

fn main() -> eyre::Result<()> {
    let filepath = "./samples/2.mp3";
    let (sample_rate, raw) = decode::decode_file(filepath)?;
    let r = raw.clone();

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

    std::thread::sleep(std::time::Duration::from_secs(200));

    Ok(())
}

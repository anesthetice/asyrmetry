// Modules
mod decode;

// Imports
use tinyaudio::prelude::*;

fn main() -> eyre::Result<()> {
    let filepath = "./samples/2.mp3";
    let (sample_rate, raw) = decode::decode_file(filepath)?;

    let params = OutputDeviceParameters {
        channels_count: 2,
        sample_rate: sample_rate as usize,
        channel_sample_count: 4410,
    };

    let _device = run_output_device(params, {
        let mut index: usize = 0;
        let mut rand: f32 = 0.0;
        move |data| {
            for samples in data.chunks_mut(params.channels_count) {
                rand = (index as u32 % sample_rate) as f32 / sample_rate as f32;
                if rand < 0.4 {
                    rand = 0.4
                }
                if rand > 0.6 {
                    rand = 0.6
                }
                samples[0] = (1.0 - rand) * raw[index];
                samples[1] = rand * raw[index];
                index += 1;
            }
        }
    })
    .unwrap();

    std::thread::sleep(std::time::Duration::from_secs(200));

    Ok(())
}

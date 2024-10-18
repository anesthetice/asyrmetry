use eyre::{Context, OptionExt};
use symphonia::{
    core::{
        audio::{AudioBufferRef, Signal},
        errors::Error,
        io::MediaSourceStream,
        probe::Hint,
    },
    default::{get_codecs, get_probe},
};

pub fn decode_file<Q: AsRef<std::path::Path>>(filepath: Q) -> eyre::Result<(u32, Vec<f32>)> {
    let file = std::fs::OpenOptions::new()
        .read(true)
        .open(filepath)
        .context("Failed to open specified filepath")?;

    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let probed = get_probe().format(
        Hint::new().with_extension("mp3"),
        mss,
        &Default::default(),
        &Default::default(),
    )?;

    let mut format = probed.format;

    let track = format
        .default_track()
        .ok_or_eyre("No default track found")?;

    let mut decoder = get_codecs().make(&track.codec_params, &Default::default())?;

    let mut raw: Vec<f32> = Vec::new();
    let sample_rate = track
        .codec_params
        .sample_rate
        .ok_or_eyre("No sample rate")?;

    loop {
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(symphonia::core::errors::Error::IoError(err))
                if err.kind() == std::io::ErrorKind::UnexpectedEof =>
            {
                break
            }
            Err(err) => Err(err)?,
        };

        match decoder.decode(&packet) {
            Ok(decoded) => {
                if let AudioBufferRef::F32(decoded) = decoded {
                    raw.extend(decoded.chan(1))
                }
            }
            Err(Error::DecodeError(_)) => (),
            Err(err) => Err(err)?,
        }
    }
    Ok((sample_rate, raw))
}

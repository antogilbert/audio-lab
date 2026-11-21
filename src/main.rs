use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Error, ErrorKind, FromSample, OutputCallbackInfo, Sample};
use std::f32::consts::PI;

const LA: f32 = 440.;
const DO: f32 = 523.25;
const MI: f32 = 329.63;

fn main() {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("failed to find device");
    let cfg = device.default_output_config().unwrap();

    let sample_rate = cfg.sample_rate() as f32;

    let la_inc: f32 = LA / sample_rate;
    let mi_inc: f32 = MI / sample_rate;
    let do_inc: f32 = DO / sample_rate;

    let mut phase = 0.;
    let mut sine_wave = move || {
        phase += la_inc;
        if phase >= 1. {
            phase -= 1.
        }
        (2. * PI * phase).sin()
    };

    let mut square_wave = move || {
        phase += mi_inc;
        if phase >= 1. {
            phase -= 1.
        }
        if phase > 0.5 {
            return 1.;
        }
        -1.
    };

    let mut sawtooth_wave = move || {
        phase += do_inc;
        if phase >= 1. {
            phase -= 1.
        }
        2. * (phase) - 1.
    };

    let err_cb = |err: Error| match err.kind() {
        ErrorKind::DeviceChanged | ErrorKind::RealtimeDenied => {
            eprintln!("{err}")
        }
        _ => eprintln!("Stream error: {err}"),
    };

    let channels = cfg.channels() as usize;
    let sine_audio_cb = move |output: &mut [f32], _info: &OutputCallbackInfo| {
        write_samples(output, channels, &mut sine_wave)
    };

    let square_audio_cb = move |output: &mut [f32], _info: &OutputCallbackInfo| {
        write_samples(output, channels, &mut square_wave)
    };

    let sawtooth_audio_cb = move |output: &mut [f32], _info: &OutputCallbackInfo| {
        write_samples(output, channels, &mut sawtooth_wave)
    };

    let sine_stream = device
        .build_output_stream(cfg.into(), sine_audio_cb, err_cb, None)
        .unwrap();

    let square_stream = device
        .build_output_stream(cfg.into(), square_audio_cb, err_cb, None)
        .unwrap();

    let sawtooth_stream = device
        .build_output_stream(cfg.into(), sawtooth_audio_cb, err_cb, None)
        .unwrap();

    sine_stream.play().unwrap();
    square_stream.play().unwrap();
    sawtooth_stream.play().unwrap();

    std::thread::sleep(std::time::Duration::from_secs(4));

    sine_stream.pause().unwrap();
    square_stream.pause().unwrap();
    sawtooth_stream.pause().unwrap();
}

fn write_samples<T>(output: &mut [T], channels: usize, next_wave_value: &mut dyn FnMut() -> f32)
where
    T: Sample + FromSample<f32>,
{
    for frame in output.chunks_mut(channels) {
        let next_sample = T::from_sample(next_wave_value());
        for sample in frame.iter_mut() {
            *sample = next_sample;
        }
    }
}

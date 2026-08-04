use core::f32;

use hound;
use num_complex::Complex;

const TAPS: u8 = 65;
const CUTOFF: f32 = 90000.0;
const ALMOST_ZERO: f32 = 1e-4;

fn generate_lowpass_filter(sample_rate: f32) -> Vec<f32> {
    let mut indices: Vec<f32> = Vec::new();
    for i in 0..TAPS {
        indices.push(i as f32);
    }
    let m: f32 = (TAPS as f32 - 1.0) / 2.0;
    let fc_norm = CUTOFF / sample_rate;

    let mut h_sincs: Vec<f32> = Vec::new();
    for i in 0..TAPS as usize {
        let x = 2.0 * fc_norm * (indices[i] - m);
        h_sincs.push(x);
    }

    for i in 0..TAPS as usize {
        if h_sincs[i].abs() > ALMOST_ZERO {
            h_sincs[i] = 2.0 * fc_norm * (f32::sin(f32::consts::PI * h_sincs[i]) / (f32::consts::PI * h_sincs[i]))
        } else {
            h_sincs[i] = 2.0 * fc_norm;
        }
    }

    let mut hamming: Vec<f32> = Vec::new();
    for i in 0..TAPS as usize {
        let val = 0.54 - 0.46 * f32::cos((2.0 * f32::consts::PI * i as f32) / (TAPS as f32 - 1.0));
        hamming.push(val);
    }

    for i in 0..TAPS as usize {
        h_sincs[i] = h_sincs[i] * hamming[i];
    }

    let mut sum = 0.0;
    for i in 0..TAPS as usize {
        sum += h_sincs[i];
    }

    for i in 0..TAPS as usize {
        h_sincs[i] /= sum;
    }

    h_sincs
}

fn load_iq(path: &str) -> Vec<Complex<f32>> {
    let mut reader = hound::WavReader::open(path).unwrap();
    println!("{:?}", reader.spec());

    let samples: Vec<i16> = reader.samples::<i16>()
        .map(|s| s.unwrap())
        .collect();

    let iq: Vec<Complex<f32>> = samples.chunks(2).map(|c| Complex::new(c[0] as f32, c[1] as f32)).collect();
    iq
}

fn main() {
    let iq = load_iq("sample.wav");
    
    // build indices

}

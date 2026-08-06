use core::f32;

use hound;
use num_complex::Complex;
use std::fs::File;
use std::io::Write;

fn save_iq_to_csv(iq: &[Complex<f32>], path: &str) {
    let mut file = File::create(path).unwrap();
    for c in iq.iter() {
        writeln!(file, "{},{}", c.re, c.im).unwrap();
    }
}

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
    let mut filtered: Vec<Complex<f32>> = Vec::new();
    
    let coeffs = generate_lowpass_filter(187500.0);

    for i in 64..iq.len(){
        let mut sum_re = 0.0;
        let mut sum_im = 0.0;
        let idx = i - (TAPS as usize - 1);
        for j in 0..TAPS as usize {
            sum_re += iq[idx + j].re * coeffs[j];
            sum_im += iq[idx + j].im * coeffs[j];
        }
        filtered.push(Complex { re: sum_re, im: sum_im });
    }

    save_iq_to_csv(&filtered[0..1_000_000], "filtered_sample.csv");
}

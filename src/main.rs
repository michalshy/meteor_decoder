use hound;
use num_complex::Complex;

const TAPS: u8 = 65;
const CUTOFF: f32 = 90000.0;
const ALMOST_ZERO: f32 = 1e-4;

fn generate_lowpass_filter(sample_rate: f32) {
    let mut indices: Vec<f32> = Vec::new();
    for i in 0..TAPS {
        indices.push(i as f32);
    }
    let m: f32 = (TAPS as f32 - 1.0) / 2.0;
    let fc_norm = CUTOFF / sample_rate;

    let mut x_vals: Vec<f32> = Vec::new();
    for i in 0..TAPS as usize {
        let x = 2.0 * fc_norm * (indices[i] - m);
        x_vals.push(x);
    }
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

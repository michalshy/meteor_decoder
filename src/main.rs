use hound;
use num_complex::Complex;
use std::fs::File;
use std::io::Write;

fn save_constellation_csv(iq: &[Complex<f32>], path: &str, n: usize) {
    let mut file = File::create(path).unwrap();
    for c in iq.iter().take(n) {
        writeln!(file, "{},{}", c.re, c.im).unwrap();
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
    
    println!("iq samples: {}", iq.len());
    
    let max_i = iq.iter().map(|c| c.re).fold(f32::MIN, f32::max);
    let max_q = iq.iter().map(|c| c.im).fold(f32::MIN, f32::max);
    println!("max I: {}, max Q: {}", max_i, max_q);
    
    let mean_i: f32 = iq.iter().map(|c| c.re).sum::<f32>() / iq.len() as f32;
    let mean_q: f32 = iq.iter().map(|c| c.im).sum::<f32>() / iq.len() as f32;
    println!("mean I: {}, mean Q: {}", mean_i, mean_q);

    save_constellation_csv(&iq, "constellation.csv", 10000);
}

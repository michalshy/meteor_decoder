use hound;

fn main() {
    let reader = hound::WavReader::open("sample.wav").unwrap();
    println!("{:?}", reader.spec());
}

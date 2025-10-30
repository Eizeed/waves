use color_eyre::Result;
use serde_json::Value;

pub struct Fpcalc {
    pub duration: u64,
    pub fingerprint: String,
    _priv: (),
}

pub fn fpcalc(path: &str) -> Result<Fpcalc> {
    let output = std::process::Command::new("fpcalc")
        .arg(path)
        .arg("-json")
        .output()?;

    let fpcalc = String::from_utf8_lossy(&output.stdout);

    let mut value: Value = serde_json::from_str(&fpcalc)?;
    let fingerprint = value["fingerprint"].take().to_string().replace("\"", "");
    let duration = value["duration"].as_f64().unwrap().floor() as u64;

    Ok(Fpcalc {
        duration,
        fingerprint,
        _priv: (),
    })
}

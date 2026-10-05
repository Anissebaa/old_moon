use std::error::Error;
use std::process::Command;

pub fn launch_tshark(iface: &str) -> Result<(), Box<dyn Error>> {
    println!("Launching tshark on {iface} (Ctrl+C to stop)...");
    Command::new("tshark")
    .args(["-i", iface, "-V"])
    .status()?;
    Ok(())
}

pub fn launch_pyshark(iface: &str) -> Result<(), Box<dyn Error>> {
    let py = format!(
        "import pyshark; \
cap = pyshark.LiveCapture(interface='{iface}'); \
[print(p) for p in cap.sniff_continuously()]"
    );
    Command::new("python3").args(["-c", &py]).status()?;
    Ok(())
}

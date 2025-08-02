use std::time::Duration;

pub fn scaffold() -> anyhow::Result<()> {
    std::thread::sleep(Duration::from_millis(230));
    Ok(())
}

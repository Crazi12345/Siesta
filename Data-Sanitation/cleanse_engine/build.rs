
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("/home/tired_atlas/ThriveTime/protos/calendar.proto")?;
    Ok(())
}

use std::process::Command;

pub fn deploy_contract() -> std::io::Result<()> {
    let output = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .output()?;

    if !output.status.success() {
        panic!("Failed to compile the contract.");
    }

    let deploy_command = format!(
        "neutron deploy --contract ./target/wasm32-unknown-unknown/release/contract.wasm --config ./deploy/config.toml"
    );

    let output = Command::new("sh")
        .arg("-c")
        .arg(&deploy_command)
        .output()?;

    if !output.status.success() {
        panic!("Failed to deploy the contract.");
    }

    Ok(())
}

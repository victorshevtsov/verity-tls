use risc0_zkvm::ExecutorEnv;
use verity_tls::tlsn_core::presentation::Presentation;

pub(crate) fn write_input(
    presentation: &Presentation,
) -> Result<ExecutorEnv<'_>, Box<dyn std::error::Error>> {
    write_input_3(presentation)
}

#[allow(dead_code)]
fn write_input_1(
    presentation: &Presentation,
) -> Result<ExecutorEnv<'_>, Box<dyn std::error::Error>> {
    let env = ExecutorEnv::builder().write(presentation)?.build()?;

    Ok(env)
}

#[allow(dead_code)]
fn write_input_2(
    presentation: &Presentation,
) -> Result<ExecutorEnv<'_>, Box<dyn std::error::Error>> {
    let input = serde_json::to_string(presentation)?;
    let input: &[u8] = input.as_bytes();

    let env = ExecutorEnv::builder().write(&input)?.build()?;

    Ok(env)
}

#[allow(dead_code)]
fn write_input_3(
    presentation: &Presentation,
) -> Result<ExecutorEnv<'_>, Box<dyn std::error::Error>> {
    let input_bytes = bincode::serialize(&presentation)?;

    println!("Host is writing input_bytes: {}", input_bytes.len());

    let env = ExecutorEnv::builder()
        .write(&input_bytes.len())?
        .write_slice(&input_bytes)
        .build()?;

    Ok(env)
}

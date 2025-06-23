use verity_tls::tlsn_core::{presentation::Presentation, CryptoProvider};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let presentation: Presentation = serde_json::from_str(fixtures::proof::PRESENTATION_32B_FULL)?;
    let presentation_output = presentation.verify(&CryptoProvider::default())?;

    println!("server_name: {:?}", &presentation_output.server_name);
    println!(
        "connection_info: {:?}",
        &presentation_output.connection_info
    );

    if let Some(mut transcript) = presentation_output.transcript {
        transcript.set_unauthed(b'X');

        let sent = String::from_utf8(transcript.sent_unsafe().to_vec())?;
        let received = String::from_utf8(transcript.received_unsafe().to_vec())?;

        println!("sent: {}", sent);
        println!("received: {}", received);
    } else {
        println!("sent: no transcript",);
        println!("received: no transcript");
    }

    Ok(())
}

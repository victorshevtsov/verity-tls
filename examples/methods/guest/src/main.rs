use verity_tls::{
    tlsn_core::{presentation::Presentation, CryptoProvider},
    {Request, Response},
};

mod interop;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request = interop::read_request()?;

    let response = process_request(request)?;

    interop::commit_response(&response)?;

    Ok(())
}

fn process_request(request: Request) -> Result<Response, Box<dyn std::error::Error>> {
    let mut presentations = Vec::<Presentation>::new();

    for presentation in request.items.into_iter() {
        presentations.push(presentation.clone().wipe_private_data()?);

        presentation.verify_private_facets(&CryptoProvider::default())?;
    }

    Ok(Response::from(presentations))
}

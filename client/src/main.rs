use websocket::client::builder::ClientBuilder;
use websocket::dataframe::DataFrame;

fn main() -> anyhow::Result<()> {
    let mut client = ClientBuilder::new("ws://localhost:3000")?
        .connect_insecure()?;

    let dataframe = DataFrame {
        finished: true,
        reserved: [false, true, true],
        opcode: websocket::dataframe::Opcode::Text,
        data: "Zap!".to_string().into_bytes(),
    };

    client.send_dataframe(&dataframe)?;

    Ok(())
}

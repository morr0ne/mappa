use anyhow::Result as AnyResult;
use clap::{App, Arg};
use log::info;
use mappa::{types::Commands, SessionBuilder};

#[tokio::main]
async fn main() -> AnyResult<()> {
    env_logger::init();

    let matches = App::new("mappa")
        .arg(Arg::new("email").required(true))
        .arg(Arg::new("password").required(true))
        .arg(Arg::new("server").required(true))
        .get_matches();

    // Get path to the torrent
    let email = matches.value_of("email").unwrap();
    let password = matches.value_of("password").unwrap();
    let server = matches.value_of("server").unwrap();

    info!("Connecting to {}", server);

    let (mut session, response) = SessionBuilder::new().connect((server, 993)).await?;
    print!("{}", response);

    let res = session.login(email, password).await?;
    print!("{}", res);

    let res = session.send_command(Commands::NOOP).await?;
    print!("{}", res);

    // let res = session.select("inbox").await?;
    // print!("{}", res);

    let res = session.send_command(Commands::LOGOUT).await?;
    print!("{}", res);

    // let res = session.send_command(Commands::select("INBOX")).await?;
    // println!("{}", res);

    Ok(())
}

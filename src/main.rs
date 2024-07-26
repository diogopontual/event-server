use tokio::net::{TcpListener, TcpStream};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    femme::start();
    let bind_address = format!("127.0.0.1:{}", env::var("PORT").unwrap());
    log::info!("Listening on {}", bind_address);
    let listener = TcpListener::bind(bind_address).await.unwrap();
    loop{
        let (socket,_) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream){]
    let mut connection = Connection::new(socket);
}

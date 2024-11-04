use tokio::net::UdpSocket;

#[tokio::main]
async fn main() {
	let config = config::Config::builder()
		.add_source(config::File::with_name("config.toml"))
		.add_source(config::Environment::with_prefix("BEATER"))
		.build()
		.unwrap();
	let uuid = config.get_string("uuid").expect("uuid not found in config");
	let server = config.get_string("server").expect("server not found in config");
	println!("Heartbeating to {} with UUID {}", server, uuid);

	let socket = UdpSocket::bind("0.0.0.0:0").await.expect("Failed to bind to server");
	socket.connect(&server).await.expect("Failed to connect to server");
	loop {
		socket.send(uuid.as_bytes()).await.expect("Failed to send data to server");
		tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
	}
}

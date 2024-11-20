use rumqttc::{AsyncClient, EventLoop, MqttOptions, QoS};

const CHANNEL: &str = "hello/world";

#[derive(Clone)]
pub struct RClient {
	client: AsyncClient,
}

impl RClient {
	pub fn new() -> (Self, EventLoop) {
		let config = MqttOptions::new("1", "127.0.0.1", 1883);
		let (client, eventloop) = AsyncClient::new(config, 1);

		(Self { client }, eventloop)
	}

	pub async fn subcribe(&self) {
		if let Err(e) = self.client.subscribe(CHANNEL, QoS::AtLeastOnce).await {
			eprintln!("{e}");
		}
	}
}

use rumqttc::{Client, Connection, MqttOptions, QoS};

const CHANNEL: &str = "hello/world";

pub trait SendMessage {
	fn send<T: Into<Vec<u8>>>(&self, payload: T);
}

impl SendMessage for Client {
	fn send<T: Into<Vec<u8>>>(&self, payload: T) {
		self.publish(CHANNEL, QoS::AtLeastOnce, false, payload)
			.unwrap();
	}
}

pub fn new_client() -> (Client, Connection) {
	let config = MqttOptions::new("1", "127.0.0.1", 1883);
	let (client, conn) = Client::new(config, 1);

	client.subscribe(CHANNEL, QoS::AtLeastOnce).unwrap();

	(client, conn)
}

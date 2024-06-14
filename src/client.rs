use rumqttc::{Client, MqttOptions, QoS};

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

pub fn run_client() {
	let config = MqttOptions::new("1", "127.0.0.1", 1883);
	let (client, mut conn) = Client::new(config, 1);

	client.subscribe(CHANNEL, QoS::AtLeastOnce).unwrap();

	for msg in conn.iter() {
		println!("{:?}", msg);
	}
}

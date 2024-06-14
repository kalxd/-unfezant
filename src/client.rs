use rumqttc::{Client, MqttOptions, QoS};

const CHANNEL: &str = "hello/world";

pub fn run_client() {
	let config = MqttOptions::new("1", "127.0.0.1", 1883);
	let (client, mut conn) = Client::new(config, 1);

	client.subscribe(CHANNEL, QoS::AtLeastOnce).unwrap();

	/*
		std::thread::spawn(move || {
			for _ in 0..10 {
				client
					.publish(CHANNEL, QoS::AtLeastOnce, false, [1, 2, 3])
					.unwrap();
			}
	});
		*/

	for msg in conn.iter() {
		println!("{:?}", msg);
	}
}

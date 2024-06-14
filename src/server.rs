use rumqttd::{Broker, Config, ConnectionSettings, RouterConfig, ServerSettings};
use std::{collections::HashMap, net::SocketAddr, str::FromStr};

fn build_config() -> Config {
	let mut v4: HashMap<String, ServerSettings> = HashMap::new();
	v4.insert(
		"1".into(),
		ServerSettings {
			name: "v4-1".into(),
			listen: SocketAddr::from_str("127.0.0.1:1883").unwrap(),
			next_connection_delay_ms: 1,
			connections: ConnectionSettings {
				connection_timeout_ms: 60000,
				max_payload_size: 20480,
				max_inflight_count: 100,
				dynamic_filters: true,
				auth: None,
				external_auth: None,
			},
			tls: None,
		},
	);

	Config {
		id: 0,
		router: RouterConfig {
			max_connections: 10010,
			max_outgoing_packet_count: 200,
			max_segment_size: 104857600,
			max_segment_count: 10,
			..Default::default()
		},
		v4: Some(v4),
		..Default::default()
	}
}

pub fn run_server() {
	let config = build_config();
	let mut broker = Broker::new(config);
	broker.start().unwrap();
}

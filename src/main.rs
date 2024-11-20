use client::SendMessage;
use gtk4::{
	prelude::{ApplicationExt, ApplicationExtManual, BoxExt, GtkWindowExt},
	Application, ApplicationWindow, Box as GtkBox, Orientation,
};
use rumqttc::{Event, Packet};

mod client;
mod server;
mod widget;

use core::str;
use std::thread;

fn setup_ui(app: &Application) {
	thread::spawn(|| server::run_server());

	let layout = GtkBox::builder()
		.orientation(Orientation::Vertical)
		.spacing(10)
		.build();

	let log_view = widget::LogView::new();
	layout.append(&log_view.container);

	let messager = widget::SendMessager::new();
	layout.append(&messager.container);

	let window = ApplicationWindow::builder()
		.application(app)
		.title("mqtt控制台")
		.default_width(800)
		.default_height(600)
		.child(&layout)
		.build();

	window.present();

	let (client, mut conn) = client::new_client();
	thread::spawn({
		move || {
			conn.iter()
				.filter_map(|x| x.ok())
				.filter_map(|x| match x {
					Event::Incoming(m) => Some(m),
					_ => None,
				})
				.filter_map(|x| match x {
					Packet::Publish(p) => Some(p),
					_ => None,
				})
				.for_each(|x| {
					let msg = format!("接收到：{:?}", str::from_utf8(&x.payload));
					dbg!(msg);
				});
		}
	});

	messager.connect_send_message(move |msg| {
		client.send(msg);
	});
}

fn main() {
	let app = Application::builder()
		.application_id("person.xgley.unfezant")
		.build();

	app.connect_activate(setup_ui);

	app.run();
}

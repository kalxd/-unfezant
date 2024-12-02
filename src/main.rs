use client::SendMessage;
use futures::{channel::mpsc, future::ready, StreamExt};
use gtk::{
	glib,
	prelude::{ApplicationExt, ApplicationExtManual, BoxExt, WidgetExt},
	Application, ApplicationWindow, Box as GtkBox, Orientation,
};
use rumqttc::{Event, Packet};

use std::{str::from_utf8, thread};

mod client;
mod server;
mod widget;

enum MessageAction {
	Packet(String),
}

fn setup_ui(app: &Application) {
	let (mut sx, rx) = mpsc::channel::<MessageAction>(10);

	let layout = GtkBox::builder()
		.orientation(Orientation::Vertical)
		.spacing(10)
		.build();

	let log_view = widget::LogView::new();
	layout.pack_start(&log_view.container, true, true, 0);

	let messager = widget::SendMessager::new();
	layout.pack_start(&messager.container, false, false, 0);

	let window = ApplicationWindow::builder()
		.application(app)
		.title("mqtt控制台")
		.default_width(800)
		.default_height(600)
		.child(&layout)
		.build();

	window.show_all();

	let (client, mut conn) = client::new_client();

	thread::spawn(move || {
		conn.iter()
			.filter_map(|x| x.ok())
			.inspect({
				let mut sx = sx.clone();
				move |x| {
					let payload = format!("{:?}\n", x);
					sx.try_send(MessageAction::Packet(payload)).unwrap();
				}
			})
			.filter_map(|x| match x {
				Event::Incoming(Packet::Publish(m)) => Some(m),
				_ => None,
			})
			.filter_map(|x| {
				from_utf8(&x.payload)
					.map(|s| format!("接受到：\n{:?}\n", s))
					.ok()
			})
			.for_each(|msg| {
				sx.try_send(MessageAction::Packet(msg.to_string())).unwrap();
			});
	});

	glib::MainContext::default().spawn_local(async move {
		rx.for_each(|MessageAction::Packet(packet)| {
			log_view.append_log(&packet);
			ready(())
		})
		.await;
	});

	messager.connect_send_message(move |msg| {
		client.send(msg);
	});
}

fn main() {
	thread::spawn(|| server::run_server());

	let app = Application::builder()
		.application_id("person.xgley.unfezant")
		.build();

	app.connect_activate(setup_ui);

	app.run();
}

use gtk4::{
	glib,
	prelude::{ApplicationExt, ApplicationExtManual, BoxExt, GtkWindowExt},
	Application, ApplicationWindow, Box as GtkBox, Orientation,
};

mod client;
mod server;
mod widget;

use std::thread;

fn setup_ui(app: &Application) {
	let (client, mut eventloop) = client::RClient::new();

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

	messager.connect_send_message(|msg| {
		dbg!(msg);
	});

	glib::MainContext::default().spawn(async move {
		client.subcribe().await;
	});

	glib::MainContext::default().spawn_local(async move {
		loop {
			if let Ok(evt) = eventloop.poll().await {
				dbg!(evt);
			}
		}
	});
}

fn main() {
	let srv = thread::spawn(|| server::run_server);

	let app = Application::builder()
		.application_id("person.xgley.unfezant")
		.build();

	app.connect_activate(setup_ui);

	app.run();

	srv.join().unwrap();
}

use gtk4::{
	prelude::{ApplicationExt, ApplicationExtManual, BoxExt, GtkWindowExt},
	Application, ApplicationWindow, Box as GtkBox, Orientation,
};

mod server;
mod widget;

fn setup_ui(app: &Application) {
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
}

fn main() {
	let app = Application::builder()
		.application_id("person.xgley.unfezant")
		.build();

	app.connect_activate(setup_ui);

	app.run();
}

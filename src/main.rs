use gtk4::{
	prelude::{ApplicationExt, ApplicationExtManual, GtkWindowExt},
	Application, ApplicationWindow,
};

fn setup_ui(app: &Application) {
	let window = ApplicationWindow::builder()
		.application(app)
		.default_width(800)
		.default_height(600)
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

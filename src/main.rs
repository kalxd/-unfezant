use gtk::{
	prelude::{ApplicationExt, ApplicationExtManual, WidgetExt},
	Application, ApplicationWindow,
};

fn build_ui(app: &Application) {
	let window = ApplicationWindow::builder().application(app).build();

	window.show_all();
}

fn main() {
	let app = Application::builder()
		.application_id("person.xgley.unfezant")
		.build();

	app.connect_activate(build_ui);
	app.run();
}

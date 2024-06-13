use gtk::{
	prelude::{ApplicationExt, ApplicationExtManual, ContainerExt, WidgetExt},
	Application, ApplicationWindow, Box as GtkBox,
};

fn build_ui(app: &Application) {
	let window = ApplicationWindow::builder().application(app).build();
	let layout = GtkBox::builder().build();
	window.add(&layout);
	window.show_all();
}

fn main() {
	let app = Application::builder()
		.application_id("person.xgley.unfezant")
		.build();

	app.connect_activate(build_ui);
	app.run();
}

use gtk4::{
	prelude::{ApplicationExt, ApplicationExtManual, BoxExt, GtkWindowExt, WidgetExt},
	Application, ApplicationWindow, Box as GtkBox, Button, Orientation, TextView,
};

fn setup_ui(app: &Application) {
	let layout = GtkBox::builder()
		.orientation(Orientation::Vertical)
		.spacing(10)
		.build();

	let text_view = TextView::new();
	text_view.set_vexpand(true);
	layout.append(&text_view);

	let send_btn = Button::with_label("发送");
	layout.append(&send_btn);

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

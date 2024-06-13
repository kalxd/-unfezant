use gtk::{
	prelude::{ApplicationExt, ApplicationExtManual, BoxExt, ContainerExt, WidgetExt},
	Application, ApplicationWindow, Box as GtkBox, Button, Entry, Orientation, TextView,
};

fn build_ui(app: &Application) {
	let window = ApplicationWindow::builder()
		.application(app)
		.width_request(600)
		.height_request(400)
		.build();
	let layout = GtkBox::builder().orientation(Orientation::Vertical).build();

	let text_view = TextView::new();
	layout.pack_start(&text_view, true, true, 10);

	let send_layout = GtkBox::builder().build();
	let entry = Entry::new();
	send_layout.pack_start(&entry, true, true, 0);
	let btn = Button::builder().label("发送").build();
	send_layout.pack_start(&btn, false, true, 10);
	layout.pack_start(&send_layout, false, true, 0);

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

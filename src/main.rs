use gtk::{
	prelude::{ApplicationExt, ApplicationExtManual, BoxExt, ContainerExt, WidgetExt},
	Application, ApplicationWindow, Box as GtkBox, Button, Entry, Frame, Orientation, TextView,
};

fn build_ui(app: &Application) {
	let window = ApplicationWindow::builder()
		.application(app)
		.width_request(600)
		.height_request(400)
		.build();
	let layout = GtkBox::builder()
		.orientation(Orientation::Vertical)
		.spacing(10)
		.build();

	let view_frame = Frame::builder().label("日志").build();
	let text_view = TextView::new();
	view_frame.add(&text_view);
	layout.pack_start(&view_frame, true, true, 0);

	let send_layout = GtkBox::builder().spacing(10).build();
	let entry = Entry::new();
	send_layout.pack_start(&entry, true, true, 0);
	let btn = Button::builder().label("发送").build();
	send_layout.pack_start(&btn, false, true, 0);
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

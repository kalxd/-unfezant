use gtk4::{
	prelude::{BoxExt, WidgetExt},
	Box as GtkBox, Button, Entry, Frame, ScrolledWindow, TextBuffer, TextView,
};

pub struct LogView {
	pub container: Frame,
	text_buf: TextBuffer,
}

impl LogView {
	pub fn new() -> Self {
		let text_buf = TextBuffer::new(None);
		let view = TextView::builder()
			.buffer(&text_buf)
			.editable(false)
			.build();

		let scroll_window = ScrolledWindow::builder().child(&view).build();

		let container = Frame::builder()
			.label("log view")
			.vexpand(true)
			.child(&scroll_window)
			.build();

		Self {
			container,
			text_buf,
		}
	}
}

pub struct SendMessager {
	pub container: GtkBox,
}

impl SendMessager {
	pub fn new() -> Self {
		let container = GtkBox::new(gtk4::Orientation::Horizontal, 10);

		let text_entry = Entry::new();
		text_entry.set_hexpand(true);
		container.append(&text_entry);

		let send_btn = Button::with_label("发送");
		container.append(&send_btn);

		Self { container }
	}
}

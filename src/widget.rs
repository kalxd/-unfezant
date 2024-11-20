use gtk4::{
	prelude::{BoxExt, ButtonExt, EditableExt, EntryExt, WidgetExt},
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
	text_entry: Entry,
	send_btn: Button,
}

impl SendMessager {
	pub fn new() -> Self {
		let container = GtkBox::new(gtk4::Orientation::Horizontal, 10);

		let text_entry = Entry::new();
		text_entry.set_hexpand(true);
		container.append(&text_entry);

		let send_btn = Button::with_label("发送");
		container.append(&send_btn);

		Self {
			container,
			text_entry,
			send_btn,
		}
	}

	pub fn connect_send_message<F>(&self, f: F)
	where
		F: Fn(String) + Clone + 'static,
	{
		self.send_btn.connect_clicked({
			let f = f.clone();
			let entry = self.text_entry.clone();
			move |_| {
				let value = entry.text();
				f(value.to_string());
			}
		});

		self.text_entry.connect_activate(move |entry| {
			let value = entry.text();
			f(value.to_string())
		});
	}
}

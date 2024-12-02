use gtk::{
	prelude::{BoxExt, ButtonExt, EntryExt, TextBufferExt, WidgetExt},
	Box as GtkBox, Button, Entry, Frame, ScrolledWindow, TextBuffer, TextView,
};

#[derive(Clone)]
pub struct LogView {
	pub container: Frame,
	text_buf: TextBuffer,
}

impl LogView {
	pub fn new() -> Self {
		let text_buf = TextBuffer::builder().build();
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

	pub fn append_log(&self, log: &str) {
		let mut iter = self.text_buf.end_iter();
		self.text_buf.insert(&mut iter, log);
	}
}

pub struct SendMessager {
	pub container: GtkBox,
	text_entry: Entry,
	send_btn: Button,
}

trait SendTextExt {
	fn try_get_text(&self) -> Option<String>;
}

impl SendTextExt for Entry {
	fn try_get_text(&self) -> Option<String> {
		let text = self.text();
		let text = text.trim();

		if text.is_empty() {
			return None;
		}

		self.set_text("");
		Some(text.to_string())
	}
}

impl SendMessager {
	pub fn new() -> Self {
		let container = GtkBox::new(gtk::Orientation::Horizontal, 10);

		let text_entry = Entry::new();
		text_entry.set_hexpand(true);
		container.pack_start(&text_entry, true, true, 0);

		let send_btn = Button::with_label("发送");
		container.pack_start(&send_btn, false, false, 0);

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
				if let Some(msg) = entry.try_get_text() {
					f(msg);
				}
			}
		});

		self.text_entry.connect_activate(move |entry| {
			if let Some(msg) = entry.try_get_text() {
				f(msg);
			}
		});
	}
}

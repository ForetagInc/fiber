use contacts::Contact;

#[derive(Debug)]
pub struct FocusManager {
	pub active_mode: Option<FocusMode>,
	pub modes: Vec<FocusMode>,
}

impl FocusManager {
	pub fn new(modes: Vec<FocusMode>) -> Self {
		Self {
			active_mode: None,
			modes,
		}
	}

	pub fn set_mode(&mut self, mode: FocusMode) {
		self.active_mode = Some(mode);
	}

	pub fn add_mode(&mut self, mode: FocusMode) {
		self.modes.push(mode);
	}
}

#[derive(Debug)]
pub struct FocusMode {
	pub name: String,
	pub allowed_people: Option<Vec<Contact>>,
	pub silenced_people: Option<Vec<Contact>>,
	pub deletable: bool,
}

impl FocusMode {
	pub fn new(name: &str) -> Self {
		Self {
			name: name.to_string(),
			allowed_people: None,
			silenced_people: None,
			deletable: false,
		}
	}

	pub fn new_deletable(name: &str) -> Self {
		Self {
			name: name.to_string(),
			allowed_people: None,
			silenced_people: None,
			deletable: true,
		}
	}

	pub fn is_deletable(&self) -> bool {
		self.deletable
	}
}

use contacts::Contact;

#[derive(Debug)]
pub struct FocusMode {
	pub name: String,
	pub allowed_people: Option<Vec<Contact>>,
	pub silenced_people: Option<Vec<Contact>>
}

impl FocusMode {
	pub fn new(name: &str) -> Self {
		Self {
			name: name.to_string(),
			allowed_people: None,
			silenced_people: None
		}
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProfileKind {
	User,
	Developer,
	Hacker,
}

impl ProfileKind {
	pub const ALL: [Self; 3] = [Self::User, Self::Developer, Self::Hacker];

	pub const fn as_str(self) -> &'static str {
		match self {
			Self::User => "User",
			Self::Developer => "Developer",
			Self::Hacker => "Hacker",
		}
	}

	pub const fn auto_connects_tor(self) -> bool {
		match self {
			Self::User | Self::Developer => false,
			Self::Hacker => true,
		}
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Profile {
	pub kind: ProfileKind,
	pub name: &'static str,
	pub auto_connects_tor: bool,
}

impl Profile {
	pub const fn new(kind: ProfileKind) -> Self {
		Self {
			kind,
			name: kind.as_str(),
			auto_connects_tor: kind.auto_connects_tor(),
		}
	}
}

#[derive(Debug, Default)]
pub struct ProfileManager {
	pub active_profile: Option<Profile>,
	pub profiles: Vec<Profile>,
}

impl ProfileManager {
	pub fn new() -> Self {
		Self {
			active_profile: None,
			profiles: ProfileKind::ALL.into_iter().map(Profile::new).collect(),
		}
	}
}

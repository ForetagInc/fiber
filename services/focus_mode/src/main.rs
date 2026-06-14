use focus_mode::{FocusManager, FocusMode};

pub fn main() {
	let modes = default_modes();
	let focus_manager = FocusManager::new(modes);
}

pub fn default_modes() -> Vec<FocusMode> {
	vec![
		FocusMode::new("Do Not Disturb"),
		FocusMode::new_deletable("Fitness"),
		FocusMode::new_deletable("Work"),
		FocusMode::new_deletable("Driving"),
		FocusMode::new_deletable("Gaming"),
		FocusMode::new_deletable("Mindfulness"),
		FocusMode::new_deletable("Reading"),
		FocusMode::new_deletable("Sleep"),
	]
}

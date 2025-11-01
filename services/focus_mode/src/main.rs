use focus_mode::FocusMode;

pub fn main() {}

fn default_modes() -> Vec<FocusMode> {
	vec![
		FocusMode::new("Fitness"),
		FocusMode::new("Work"),
		FocusMode::new("Driving"),
		FocusMode::new("Gaming"),
		FocusMode::new("Mindfulness"),
		FocusMode::new("Reading"),
		FocusMode::new("Sleep")
	]
}

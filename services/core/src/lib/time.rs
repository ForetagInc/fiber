use super::profile::ProfileKind;

pub const DEFAULT_NTP_SERVER: &str = "time.cloudflare.com";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimeProfile {
	pub ntp_server: &'static str,
}

impl TimeProfile {
	pub const fn for_profile(profile: ProfileKind) -> Self {
		match profile {
			ProfileKind::User | ProfileKind::Developer | ProfileKind::Hacker => Self {
				ntp_server: DEFAULT_NTP_SERVER,
			},
		}
	}
}

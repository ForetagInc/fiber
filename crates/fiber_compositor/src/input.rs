use crate::security::ProtectedResource;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputClass {
	Keyboard,
	Mouse,
	Trackpad,
	Touch,
	Stylus,
	Gamepad,
	Remote,
	XrController,
	HandTracking,
	Gaze,
	HeadPose,
	VoiceCommand,
	SpatialPointerRay,
}

impl InputClass {
	pub const fn all() -> &'static [Self] {
		&[
			Self::Keyboard,
			Self::Mouse,
			Self::Trackpad,
			Self::Touch,
			Self::Stylus,
			Self::Gamepad,
			Self::Remote,
			Self::XrController,
			Self::HandTracking,
			Self::Gaze,
			Self::HeadPose,
			Self::VoiceCommand,
			Self::SpatialPointerRay,
		]
	}

	pub const fn requires_permission(self) -> bool {
		matches!(
			self,
			Self::HandTracking
				| Self::Gaze | Self::HeadPose
				| Self::VoiceCommand
				| Self::SpatialPointerRay
		)
	}

	pub const fn protected_resource(self) -> Option<ProtectedResource> {
		match self {
			Self::HandTracking => Some(ProtectedResource::HandTracking),
			Self::Gaze => Some(ProtectedResource::GazeTarget),
			Self::HeadPose => Some(ProtectedResource::HeadPose),
			Self::VoiceCommand => Some(ProtectedResource::Microphone),
			Self::SpatialPointerRay => Some(ProtectedResource::SpatialMapping),
			_ => None,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InteractionPrimitive {
	Point,
	Click,
	Touch,
	Gesture,
	Scroll,
	Drag,
	Pinch,
	Grab,
	Select,
	Focus,
	Type,
	Dictate,
	SpatialRaycast,
	GazeFocus,
	ControllerAction,
	HandPoseAction,
}

impl InteractionPrimitive {
	pub const fn all() -> &'static [Self] {
		&[
			Self::Point,
			Self::Click,
			Self::Touch,
			Self::Gesture,
			Self::Scroll,
			Self::Drag,
			Self::Pinch,
			Self::Grab,
			Self::Select,
			Self::Focus,
			Self::Type,
			Self::Dictate,
			Self::SpatialRaycast,
			Self::GazeFocus,
			Self::ControllerAction,
			Self::HandPoseAction,
		]
	}
}

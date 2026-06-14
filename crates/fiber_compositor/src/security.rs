#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProtectedResource {
	HeadPose,
	HandTracking,
	EyeTracking,
	GazeTarget,
	SpatialMesh,
	RoomMapping,
	CameraPassthrough,
	Microphone,
	SpatialAnchors,
	BodyTracking,
	ControllerIdentity,
	SpatialMapping,
	LocalEnvironmentUnderstanding,
	Camera,
	Location,
	ScreenCapture,
}

impl ProtectedResource {
	pub const fn all() -> &'static [Self] {
		&[
			Self::HeadPose,
			Self::HandTracking,
			Self::EyeTracking,
			Self::GazeTarget,
			Self::SpatialMesh,
			Self::RoomMapping,
			Self::CameraPassthrough,
			Self::Microphone,
			Self::SpatialAnchors,
			Self::BodyTracking,
			Self::ControllerIdentity,
			Self::SpatialMapping,
			Self::LocalEnvironmentUnderstanding,
			Self::Camera,
			Self::Location,
			Self::ScreenCapture,
		]
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PermissionGrant {
	Denied,
	Temporary,
	Session,
	Persistent,
	ManagedByPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenXrStrategy {
	pub enabled: bool,
	pub initial_runtime: OpenXrRuntime,
	pub allow_vendor_runtimes: bool,
	pub fiber_runtime_profile: bool,
}

impl Default for OpenXrStrategy {
	fn default() -> Self {
		Self {
			enabled: true,
			initial_runtime: OpenXrRuntime::Monado,
			allow_vendor_runtimes: true,
			fiber_runtime_profile: false,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OpenXrRuntime {
	Monado,
	VendorCertified,
	FiberRuntimeProfile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OpenXrCapability {
	DeviceDiscovery,
	SessionCreation,
	HeadPoseTracking,
	ViewProjectionConfiguration,
	ControllerInput,
	HandTracking,
	GazeTracking,
	SpatialReferenceSpaces,
	FrameTiming,
	SwapchainSubmission,
	ImmersivePresentation,
	PassthroughExtensions,
}

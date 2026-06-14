#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RenderTarget {
	Desktop,
	Mobile,
	Spatial,
}

impl RenderTarget {
	pub const fn all() -> &'static [Self] {
		&[Self::Desktop, Self::Mobile, Self::Spatial]
	}

	pub const fn requirements(self) -> &'static [RenderRequirement] {
		match self {
			Self::Desktop => &[
				RenderRequirement::WaylandOutputRendering,
				RenderRequirement::MultiMonitor,
				RenderRequirement::FractionalScaling,
				RenderRequirement::HighDpi,
				RenderRequirement::VariableRefreshRate,
				RenderRequirement::ScreenCaptureMediation,
				RenderRequirement::FullScreenGameOptimization,
			],
			Self::Mobile => &[
				RenderRequirement::OrientationAwareRendering,
				RenderRequirement::HighRefreshRateDisplays,
				RenderRequirement::PowerAwareFrameScheduling,
				RenderRequirement::TouchLatencyOptimization,
				RenderRequirement::AppLifecycleThrottling,
				RenderRequirement::BackgroundRenderingLimits,
			],
			Self::Spatial => &[
				RenderRequirement::OpenXrSwapchainSubmission,
				RenderRequirement::StereoRendering,
				RenderRequirement::HeadPosePrediction,
				RenderRequirement::LowLatencyFrameLoop,
				RenderRequirement::SpatialPanelComposition,
				RenderRequirement::DepthAwareComposition,
				RenderRequirement::PassthroughLayerSupport,
				RenderRequirement::ComfortConstraints,
			],
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RenderRequirement {
	WaylandOutputRendering,
	MultiMonitor,
	FractionalScaling,
	HighDpi,
	VariableRefreshRate,
	HdrRoadmap,
	FullScreenGameOptimization,
	ScreenCaptureMediation,
	OrientationAwareRendering,
	HighRefreshRateDisplays,
	PowerAwareFrameScheduling,
	TouchLatencyOptimization,
	AppLifecycleThrottling,
	BackgroundRenderingLimits,
	OpenXrSwapchainSubmission,
	StereoRendering,
	HeadPosePrediction,
	LowLatencyFrameLoop,
	LateLatching,
	Reprojection,
	SpatialPanelComposition,
	DepthAwareComposition,
	PassthroughLayerSupport,
	ComfortConstraints,
	PerformanceHud,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FrameSchedulingPolicy {
	Interactive,
	PowerAware,
	BackgroundThrottled,
	LowLatencySpatial,
}

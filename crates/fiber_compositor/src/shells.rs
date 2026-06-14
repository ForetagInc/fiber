use crate::input::InputClass;
use crate::rendering::RenderTarget;
use crate::surface::{PresentationMode, SurfaceType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShellClass {
	Desktop,
	Mobile,
	Spatial,
}

#[derive(Debug, Clone)]
pub struct ShellFrontend {
	pub class: ShellClass,
	pub name: &'static str,
	pub first_release_target: bool,
	pub surfaces: &'static [SurfaceType],
	pub presentation_modes: &'static [PresentationMode],
	pub inputs: &'static [InputClass],
	pub render_targets: &'static [RenderTarget],
}

pub fn desktop_shell() -> ShellFrontend {
	ShellFrontend {
		class: ShellClass::Desktop,
		name: "Fiber Desktop Shell",
		first_release_target: true,
		surfaces: &[
			SurfaceType::DesktopWindow,
			SurfaceType::SystemOverlay,
			SurfaceType::NotificationSurface,
			SurfaceType::PermissionPrompt,
			SurfaceType::GamingFullScreenSurface,
			SurfaceType::PwaAppSurface,
			SurfaceType::BrowserSurface,
			SurfaceType::XWaylandCompatibilitySurface,
		],
		presentation_modes: &[
			PresentationMode::DesktopWindowed,
			PresentationMode::DesktopFullScreen,
			PresentationMode::GameMode,
			PresentationMode::KioskMode,
		],
		inputs: &[
			InputClass::Keyboard,
			InputClass::Mouse,
			InputClass::Trackpad,
			InputClass::Touch,
			InputClass::Stylus,
			InputClass::Gamepad,
		],
		render_targets: &[RenderTarget::Desktop],
	}
}

pub fn mobile_shell() -> ShellFrontend {
	ShellFrontend {
		class: ShellClass::Mobile,
		name: "Fiber Mobile Shell",
		first_release_target: false,
		surfaces: &[
			SurfaceType::MobileFullScreenSurface,
			SurfaceType::MobileSheet,
			SurfaceType::TabletFloatingWindow,
			SurfaceType::SystemOverlay,
			SurfaceType::NotificationSurface,
			SurfaceType::PermissionPrompt,
			SurfaceType::LockScreenSurface,
			SurfaceType::PwaAppSurface,
			SurfaceType::BrowserSurface,
		],
		presentation_modes: &[
			PresentationMode::MobileCompact,
			PresentationMode::MobileFullScreen,
			PresentationMode::TabletAdaptive,
			PresentationMode::KioskMode,
		],
		inputs: &[
			InputClass::Touch,
			InputClass::Stylus,
			InputClass::Keyboard,
			InputClass::Gamepad,
			InputClass::VoiceCommand,
		],
		render_targets: &[RenderTarget::Mobile],
	}
}

pub fn spatial_shell() -> ShellFrontend {
	ShellFrontend {
		class: ShellClass::Spatial,
		name: "Fiber Spatial Shell",
		first_release_target: false,
		surfaces: &[
			SurfaceType::SpatialPanel,
			SurfaceType::SpatialImmersiveLayer,
			SurfaceType::SystemOverlay,
			SurfaceType::NotificationSurface,
			SurfaceType::PermissionPrompt,
			SurfaceType::PwaAppSurface,
			SurfaceType::BrowserSurface,
			SurfaceType::XWaylandCompatibilitySurface,
		],
		presentation_modes: &[
			PresentationMode::SpatialPanel,
			PresentationMode::SpatialImmersive,
			PresentationMode::KioskMode,
		],
		inputs: &[
			InputClass::XrController,
			InputClass::HandTracking,
			InputClass::Gaze,
			InputClass::HeadPose,
			InputClass::VoiceCommand,
			InputClass::SpatialPointerRay,
			InputClass::Keyboard,
			InputClass::Gamepad,
		],
		render_targets: &[RenderTarget::Spatial],
	}
}

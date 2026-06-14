use crate::shells::ShellClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SurfaceType {
	DesktopWindow,
	MobileFullScreenSurface,
	MobileSheet,
	TabletFloatingWindow,
	SpatialPanel,
	SpatialImmersiveLayer,
	SystemOverlay,
	NotificationSurface,
	PermissionPrompt,
	LockScreenSurface,
	GamingFullScreenSurface,
	PwaAppSurface,
	BrowserSurface,
	XWaylandCompatibilitySurface,
}

impl SurfaceType {
	pub const fn all() -> &'static [Self] {
		&[
			Self::DesktopWindow,
			Self::MobileFullScreenSurface,
			Self::MobileSheet,
			Self::TabletFloatingWindow,
			Self::SpatialPanel,
			Self::SpatialImmersiveLayer,
			Self::SystemOverlay,
			Self::NotificationSurface,
			Self::PermissionPrompt,
			Self::LockScreenSurface,
			Self::GamingFullScreenSurface,
			Self::PwaAppSurface,
			Self::BrowserSurface,
			Self::XWaylandCompatibilitySurface,
		]
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PresentationMode {
	DesktopWindowed,
	DesktopFullScreen,
	MobileCompact,
	MobileFullScreen,
	TabletAdaptive,
	SpatialPanel,
	SpatialImmersive,
	GameMode,
	KioskMode,
}

impl PresentationMode {
	pub const fn all() -> &'static [Self] {
		&[
			Self::DesktopWindowed,
			Self::DesktopFullScreen,
			Self::MobileCompact,
			Self::MobileFullScreen,
			Self::TabletAdaptive,
			Self::SpatialPanel,
			Self::SpatialImmersive,
			Self::GameMode,
			Self::KioskMode,
		]
	}

	pub const fn shell_class(self) -> ShellClass {
		match self {
			Self::DesktopWindowed | Self::DesktopFullScreen | Self::GameMode => ShellClass::Desktop,
			Self::MobileCompact | Self::MobileFullScreen | Self::TabletAdaptive => {
				ShellClass::Mobile
			}
			Self::SpatialPanel | Self::SpatialImmersive => ShellClass::Spatial,
			Self::KioskMode => ShellClass::Desktop,
		}
	}
}

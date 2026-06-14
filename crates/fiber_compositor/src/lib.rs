//! Fiber Compositor architecture contracts.
//!
//! The compositor is intentionally modeled as a shared core with shell
//! frontends. The desktop shell is the first product target, while mobile and
//! spatial shells remain first-class architectural targets.

pub mod core;
pub mod input;
pub mod rendering;
pub mod security;
pub mod shells;
pub mod surface;
pub mod xr;

pub use crate::core::{CompositorConfig, CompositorPlatform};
pub use crate::input::{InputClass, InteractionPrimitive};
pub use crate::rendering::{FrameSchedulingPolicy, RenderRequirement, RenderTarget};
pub use crate::security::{PermissionGrant, ProtectedResource};
pub use crate::shells::{ShellClass, ShellFrontend};
pub use crate::surface::{PresentationMode, SurfaceType};
pub use crate::xr::{OpenXrRuntime, OpenXrStrategy};

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn default_platform_supports_all_shell_classes() {
		let platform = CompositorPlatform::default();

		assert!(platform.supports_shell(ShellClass::Desktop));
		assert!(platform.supports_shell(ShellClass::Mobile));
		assert!(platform.supports_shell(ShellClass::Spatial));
	}

	#[test]
	fn sensitive_spatial_inputs_require_permissions() {
		assert!(InputClass::Gaze.requires_permission());
		assert!(InputClass::HandTracking.requires_permission());
		assert!(InputClass::HeadPose.requires_permission());
		assert!(!InputClass::Keyboard.requires_permission());
	}

	#[test]
	fn surface_modes_map_to_shell_classes() {
		assert_eq!(
			PresentationMode::DesktopWindowed.shell_class(),
			ShellClass::Desktop
		);
		assert_eq!(
			PresentationMode::MobileFullScreen.shell_class(),
			ShellClass::Mobile
		);
		assert_eq!(
			PresentationMode::SpatialPanel.shell_class(),
			ShellClass::Spatial
		);
	}
}

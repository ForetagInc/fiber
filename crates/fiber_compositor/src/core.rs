use crate::input::InputClass;
use crate::rendering::RenderTarget;
use crate::security::ProtectedResource;
use crate::shells::{ShellClass, ShellFrontend, desktop_shell, mobile_shell, spatial_shell};
use crate::surface::SurfaceType;
use crate::xr::OpenXrStrategy;

#[derive(Debug, Clone)]
pub struct CompositorConfig {
	pub product_shell: ShellClass,
	pub openxr: OpenXrStrategy,
	pub xwayland_compatibility: bool,
	pub multi_session: bool,
}

impl Default for CompositorConfig {
	fn default() -> Self {
		Self {
			product_shell: ShellClass::Desktop,
			openxr: OpenXrStrategy::default(),
			xwayland_compatibility: true,
			multi_session: true,
		}
	}
}

#[derive(Debug, Clone)]
pub struct CompositorPlatform {
	pub config: CompositorConfig,
	pub shells: Vec<ShellFrontend>,
	pub surface_types: Vec<SurfaceType>,
	pub input_classes: Vec<InputClass>,
	pub render_targets: Vec<RenderTarget>,
	pub protected_resources: Vec<ProtectedResource>,
}

impl Default for CompositorPlatform {
	fn default() -> Self {
		Self::new(CompositorConfig::default())
	}
}

impl CompositorPlatform {
	pub fn new(config: CompositorConfig) -> Self {
		Self {
			config,
			shells: vec![desktop_shell(), mobile_shell(), spatial_shell()],
			surface_types: SurfaceType::all().to_vec(),
			input_classes: InputClass::all().to_vec(),
			render_targets: RenderTarget::all().to_vec(),
			protected_resources: ProtectedResource::all().to_vec(),
		}
	}

	pub fn supports_shell(&self, shell: ShellClass) -> bool {
		self.shells.iter().any(|frontend| frontend.class == shell)
	}

	pub fn shell(&self, shell: ShellClass) -> Option<&ShellFrontend> {
		self.shells.iter().find(|frontend| frontend.class == shell)
	}

	pub fn protected_input_classes(&self) -> impl Iterator<Item = InputClass> + '_ {
		self.input_classes
			.iter()
			.copied()
			.filter(|input| input.requires_permission())
	}
}

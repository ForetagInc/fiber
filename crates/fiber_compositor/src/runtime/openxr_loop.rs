use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct OpenXrLoopConfig {
	pub required: bool,
	pub runtime_manifest: Option<PathBuf>,
	pub frame_interval: Duration,
	pub max_frames: Option<u64>,
}

impl OpenXrLoopConfig {
	pub fn new(required: bool, runtime_manifest: Option<PathBuf>, max_frames: Option<u64>) -> Self {
		Self {
			required,
			runtime_manifest,
			frame_interval: Duration::from_micros(11_111),
			max_frames,
		}
	}
}

#[derive(Debug)]
pub struct OpenXrLoopReport {
	pub runtime_available: bool,
	pub instance_created: bool,
	pub system_detected: bool,
	pub frames: u64,
}

impl OpenXrLoopReport {
	pub fn summary(&self) -> String {
		format!(
			"runtime_available={} instance_created={} system_detected={} frames={}",
			self.runtime_available, self.instance_created, self.system_detected, self.frames
		)
	}
}

pub fn run_openxr_loop(config: OpenXrLoopConfig) -> Result<OpenXrLoopReport, String> {
	if let Some(runtime_manifest) = &config.runtime_manifest {
		if !runtime_manifest.is_file() && config.required {
			return Err(format!(
				"OpenXR runtime manifest does not exist: {}",
				runtime_manifest.display()
			));
		}
	}

	let entry = match unsafe { openxr::Entry::load() } {
		Ok(entry) => entry,
		Err(error) if config.required => {
			return Err(format!("failed to load OpenXR loader: {error}"));
		}
		Err(error) => {
			println!("fiber-compositor: OpenXR loader unavailable: {error}");
			return Ok(synthetic_frame_loop(config, false, false));
		}
	};

	let available_extensions = entry
		.enumerate_extensions()
		.map_err(|error| format!("failed to enumerate OpenXR extensions: {error}"))?;

	let app_info = openxr::ApplicationInfo {
		application_name: "Fiber Compositor",
		application_version: 1,
		engine_name: "Fiber",
		engine_version: 1,
		api_version: openxr::Version::new(1, 0, 0),
	};

	let instance = match entry.create_instance(&app_info, &openxr::ExtensionSet::default(), &[]) {
		Ok(instance) => instance,
		Err(error) if config.required => {
			return Err(format!("failed to create OpenXR instance: {error}"));
		}
		Err(error) => {
			println!("fiber-compositor: OpenXR instance unavailable: {error}");
			return Ok(synthetic_frame_loop(config, true, false));
		}
	};

	let runtime_name = instance
		.properties()
		.map(|properties| properties.runtime_name)
		.unwrap_or_else(|_| "unknown runtime".to_string());

	let system = instance.system(openxr::FormFactor::HEAD_MOUNTED_DISPLAY);
	let system_detected = system.is_ok();

	if let Ok(system) = system {
		let _ = instance
			.enumerate_view_configurations(system)
			.map(|configs| {
				println!("fiber-compositor: OpenXR view configurations: {configs:?}");
			});
	}

	println!(
		"fiber-compositor: OpenXR runtime '{runtime_name}' loaded; vulkan2={} hand_tracking={}",
		available_extensions.khr_vulkan_enable2, available_extensions.ext_hand_tracking
	);

	let mut event_storage = openxr::EventDataBuffer::new();
	let mut frames = 0;
	let started_at = Instant::now();

	loop {
		while let Some(event) = instance
			.poll_event(&mut event_storage)
			.map_err(|error| format!("failed to poll OpenXR event: {error}"))?
		{
			let _ = event;
			println!("fiber-compositor: OpenXR event received");
		}

		frames += 1;
		if config
			.max_frames
			.is_some_and(|max_frames| frames >= max_frames)
		{
			break;
		}

		if frames % 5400 == 0 {
			println!(
				"fiber-compositor: OpenXR frame loop alive for {:?}, frames={frames}",
				started_at.elapsed()
			);
		}

		thread::sleep(config.frame_interval);
	}

	Ok(OpenXrLoopReport {
		runtime_available: true,
		instance_created: true,
		system_detected,
		frames,
	})
}

fn synthetic_frame_loop(
	config: OpenXrLoopConfig,
	runtime_available: bool,
	instance_created: bool,
) -> OpenXrLoopReport {
	let frames = config.max_frames.unwrap_or(1);
	for frame in 0..frames {
		if frame > 0 {
			thread::sleep(config.frame_interval);
		}
	}

	OpenXrLoopReport {
		runtime_available,
		instance_created,
		system_detected: false,
		frames,
	}
}

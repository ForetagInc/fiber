use std::env;
use std::path::{Path, PathBuf};
use std::thread;

use fiber_compositor::{CompositorConfig, CompositorPlatform, OpenXrStrategy, ShellClass};

mod runtime;

use runtime::openxr_loop::{OpenXrLoopConfig, run_openxr_loop};
use runtime::wayland_server::{WaylandServerConfig, run_wayland_server};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OpenXrMode {
	Auto,
	Required,
	Disabled,
}

#[derive(Debug)]
struct RuntimeArgs {
	shell: ShellClass,
	openxr: OpenXrMode,
	once: bool,
	wayland_socket: Option<String>,
}

fn main() {
	if let Err(error) = run() {
		eprintln!("fiber-compositor: {error}");
		std::process::exit(1);
	}
}

fn run() -> Result<(), String> {
	let args = RuntimeArgs::parse(env::args().skip(1))?;
	let openxr_runtime = detect_openxr_runtime();

	if args.openxr == OpenXrMode::Required && openxr_runtime.is_none() {
		return Err("OpenXR runtime is required but no runtime manifest was found".to_string());
	}

	let platform = CompositorPlatform::new(CompositorConfig {
		product_shell: args.shell,
		openxr: OpenXrStrategy {
			enabled: args.openxr != OpenXrMode::Disabled,
			..OpenXrStrategy::default()
		},
		..CompositorConfig::default()
	});

	print_startup_status(&platform, args.openxr, openxr_runtime.as_deref());

	let wayland_config = wayland_config(&args);
	let openxr_config = OpenXrLoopConfig::new(
		args.openxr == OpenXrMode::Required,
		openxr_runtime.clone(),
		args.once.then_some(1),
	);

	if args.once {
		let wayland_report = run_wayland_server(wayland_config)?;
		let openxr_report = run_openxr_loop(openxr_config)?;
		println!(
			"fiber-compositor: Wayland report: {}",
			wayland_report.summary()
		);
		println!(
			"fiber-compositor: OpenXR report: {}",
			openxr_report.summary()
		);
		return Ok(());
	}

	let wayland_thread = thread::spawn(move || run_wayland_server(wayland_config));
	let openxr_thread = thread::spawn(move || run_openxr_loop(openxr_config));

	wayland_thread
		.join()
		.map_err(|_| "Wayland server thread panicked".to_string())??;
	openxr_thread
		.join()
		.map_err(|_| "OpenXR frame loop thread panicked".to_string())??;

	Ok(())
}

impl RuntimeArgs {
	fn parse<I>(mut args: I) -> Result<Self, String>
	where
		I: Iterator<Item = String>,
	{
		let mut runtime_args = Self {
			shell: ShellClass::Spatial,
			openxr: OpenXrMode::Auto,
			once: false,
			wayland_socket: None,
		};

		while let Some(arg) = args.next() {
			match arg.as_str() {
				"--shell" => {
					let value = args.next().ok_or("--shell requires a value")?;
					runtime_args.shell = parse_shell(&value)?;
				}
				"--openxr" => {
					let value = args.next().ok_or("--openxr requires a value")?;
					runtime_args.openxr = parse_openxr_mode(&value)?;
				}
				"--once" => {
					runtime_args.once = true;
				}
				"--wayland-socket" => {
					let value = args.next().ok_or("--wayland-socket requires a value")?;
					runtime_args.wayland_socket = Some(value);
				}
				"-h" | "--help" => {
					print_help();
					std::process::exit(0);
				}
				_ => return Err(format!("unknown argument '{arg}'")),
			}
		}

		Ok(runtime_args)
	}
}

fn wayland_config(args: &RuntimeArgs) -> WaylandServerConfig {
	let mut config = WaylandServerConfig::from_environment(args.once.then_some(1));
	if let Some(socket) = &args.wayland_socket {
		config.socket_name = socket.clone();
	}
	config
}

fn parse_shell(value: &str) -> Result<ShellClass, String> {
	match value {
		"desktop" => Ok(ShellClass::Desktop),
		"mobile" => Ok(ShellClass::Mobile),
		"spatial" => Ok(ShellClass::Spatial),
		_ => Err(format!("unsupported shell '{value}'")),
	}
}

fn parse_openxr_mode(value: &str) -> Result<OpenXrMode, String> {
	match value {
		"auto" => Ok(OpenXrMode::Auto),
		"required" => Ok(OpenXrMode::Required),
		"disabled" => Ok(OpenXrMode::Disabled),
		_ => Err(format!("unsupported OpenXR mode '{value}'")),
	}
}

fn detect_openxr_runtime() -> Option<PathBuf> {
	if let Ok(runtime_json) = env::var("XR_RUNTIME_JSON") {
		let path = PathBuf::from(runtime_json);
		if path.is_file() {
			return Some(path);
		}
	}

	[
		"/etc/xdg/openxr/1/active_runtime.json",
		"/usr/local/share/openxr/1/openxr_monado.json",
		"/usr/share/openxr/1/openxr_monado.json",
		"/usr/share/openxr/1/active_runtime.json",
	]
	.into_iter()
	.map(PathBuf::from)
	.find(|path| path.is_file())
}

fn print_startup_status(
	platform: &CompositorPlatform,
	openxr_mode: OpenXrMode,
	openxr_runtime: Option<&Path>,
) {
	println!("fiber-compositor: starting");
	println!(
		"fiber-compositor: product shell: {:?}",
		platform.config.product_shell
	);
	println!("fiber-compositor: openxr mode: {openxr_mode:?}");
	println!(
		"fiber-compositor: openxr runtime: {}",
		openxr_runtime
			.map(|path| path.display().to_string())
			.unwrap_or_else(|| "not detected".to_string())
	);
	println!(
		"fiber-compositor: supported shells: {}",
		platform
			.shells
			.iter()
			.map(|shell| shell.name)
			.collect::<Vec<_>>()
			.join(", ")
	);
	println!("fiber-compositor: bootstrap runtime is active");
}

fn print_help() {
	println!(
		"Usage: fiber-compositor [--shell desktop|mobile|spatial] [--openxr auto|required|disabled] [--wayland-socket NAME] [--once]"
	);
}

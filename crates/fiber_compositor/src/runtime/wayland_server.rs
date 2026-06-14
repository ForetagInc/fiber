use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{
	Arc,
	atomic::{AtomicUsize, Ordering},
};
use std::thread;
use std::time::{Duration, Instant};

use wayland_server::backend::{ClientData, ClientId, DisconnectReason};
use wayland_server::{Display, ListeningSocket};

#[derive(Debug, Clone)]
pub struct WaylandServerConfig {
	pub runtime_dir: PathBuf,
	pub socket_name: String,
	pub tick_interval: Duration,
	pub max_ticks: Option<u64>,
}

impl WaylandServerConfig {
	pub fn from_environment(max_ticks: Option<u64>) -> Self {
		let runtime_dir = std::env::var_os("XDG_RUNTIME_DIR")
			.map(PathBuf::from)
			.unwrap_or_else(|| PathBuf::from("/run/fiberos"));

		let socket_name =
			std::env::var("WAYLAND_DISPLAY").unwrap_or_else(|_| "wayland-0".to_string());

		Self {
			runtime_dir,
			socket_name,
			tick_interval: Duration::from_millis(5),
			max_ticks,
		}
	}

	pub fn socket_path(&self) -> PathBuf {
		let socket = Path::new(&self.socket_name);
		if socket.is_absolute() {
			socket.to_path_buf()
		} else {
			self.runtime_dir.join(socket)
		}
	}
}

#[derive(Debug)]
pub struct WaylandServerReport {
	pub socket_path: PathBuf,
	pub accepted_clients: usize,
	pub disconnected_clients: usize,
	pub ticks: u64,
}

impl WaylandServerReport {
	pub fn summary(&self) -> String {
		format!(
			"socket={} accepted_clients={} disconnected_clients={} ticks={}",
			self.socket_path.display(),
			self.accepted_clients,
			self.disconnected_clients,
			self.ticks
		)
	}
}

#[derive(Default)]
struct WaylandState {
	accepted_clients: Arc<AtomicUsize>,
	disconnected_clients: Arc<AtomicUsize>,
}

struct FiberClientData {
	accepted_clients: Arc<AtomicUsize>,
	disconnected_clients: Arc<AtomicUsize>,
}

impl ClientData for FiberClientData {
	fn initialized(&self, _client_id: ClientId) {
		self.accepted_clients.fetch_add(1, Ordering::Relaxed);
	}

	fn disconnected(&self, _client_id: ClientId, _reason: DisconnectReason) {
		self.disconnected_clients.fetch_add(1, Ordering::Relaxed);
	}
}

pub fn run_wayland_server(config: WaylandServerConfig) -> Result<WaylandServerReport, String> {
	fs::create_dir_all(&config.runtime_dir).map_err(|error| {
		format!(
			"failed to create Wayland runtime dir {}: {error}",
			config.runtime_dir.display()
		)
	})?;

	let socket_path = config.socket_path();
	let _ = fs::remove_file(&socket_path);
	let _ = fs::remove_file(socket_path.with_extension("lock"));

	let socket = ListeningSocket::bind_absolute(socket_path.clone()).map_err(|error| {
		format!(
			"failed to bind Wayland socket {}: {error}",
			socket_path.display()
		)
	})?;

	let mut display = Display::<WaylandState>::new()
		.map_err(|error| format!("failed to initialize Wayland display: {error}"))?;
	let mut state = WaylandState::default();
	let started_at = Instant::now();
	let mut ticks = 0;

	println!(
		"fiber-compositor: Wayland display listening at {}",
		socket_path.display()
	);

	loop {
		while let Some(stream) = socket
			.accept()
			.map_err(|error| format!("failed to accept Wayland client: {error}"))?
		{
			let client_data = Arc::new(FiberClientData {
				accepted_clients: Arc::clone(&state.accepted_clients),
				disconnected_clients: Arc::clone(&state.disconnected_clients),
			});

			display
				.handle()
				.insert_client(stream, client_data)
				.map_err(|error| format!("failed to insert Wayland client: {error}"))?;
		}

		display
			.dispatch_clients(&mut state)
			.map_err(|error| format!("failed to dispatch Wayland clients: {error}"))?;
		display
			.flush_clients()
			.map_err(|error| format!("failed to flush Wayland clients: {error}"))?;

		ticks += 1;
		if config.max_ticks.is_some_and(|max_ticks| ticks >= max_ticks) {
			break;
		}

		if ticks % 1200 == 0 {
			println!(
				"fiber-compositor: Wayland loop alive for {:?}, clients={} disconnected={}",
				started_at.elapsed(),
				state.accepted_clients.load(Ordering::Relaxed),
				state.disconnected_clients.load(Ordering::Relaxed)
			);
		}

		thread::sleep(config.tick_interval);
	}

	Ok(WaylandServerReport {
		socket_path,
		accepted_clients: state.accepted_clients.load(Ordering::Relaxed),
		disconnected_clients: state.disconnected_clients.load(Ordering::Relaxed),
		ticks,
	})
}

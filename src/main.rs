extern crate notify;
extern crate notify_rust;
extern crate reqwest;

use notify_rust::Notification;
use std::env;
use std::sync::mpsc::channel;
use notify::{Watcher, watcher, RecursiveMode, DebouncedEvent};
use std::time::Duration;
use std::path::PathBuf;

fn handle_added_video(path: PathBuf, kodi_url: &String) {
	let scan_command = kodi_url.to_string() + "/jsonrpc?request=VideoLibrary.Scan";

	println!("File added {:?}", path);

	let vide_name: &str = path.file_name().unwrap().to_str().unwrap();

	Notification::new()
		.summary("Kodi - New video")
		.body(vide_name)
		.show().unwrap();

	reqwest::get(&scan_command);
}

fn watch_folder(folder_name: String, kodi_url: String) {
	println!("Listening on {:?} ", folder_name);

	let (tx, rx) = channel();
	let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();

	watcher.watch(folder_name, RecursiveMode::Recursive).unwrap();

	loop {
		match rx.recv() {
			Ok(DebouncedEvent::Create(path)) => handle_added_video(path, &kodi_url),
			Ok(_) => {},
			Err(e) => println!("Watcher error: {:?}", e),
		}
	}
}

fn main() {
	let args = env::args().collect::<Vec<String>>();
	let cli = &args[0];

	if args.len() != 3 {
		println!("Error: Invalid number of arguments\n Usage: {:?} {{folder_name}} {{kodi_url}}", cli);
		std::process::exit(1);
	}

	let folder_name = &args[1];
	let kodi_url = &args[2];

	watch_folder(folder_name.to_string(), kodi_url.to_string());
}

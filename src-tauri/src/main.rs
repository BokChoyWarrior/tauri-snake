#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]
mod snake;

use snake::Game;
use std::ptr::null;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::{window, AppHandle, Event, Manager, State};
use tokio::time::sleep;

use crate::snake::Direction;

#[derive(Default)]
struct Counter(Arc<Mutex<i32>>);

fn main() {
	tauri::Builder::default()
		.setup(|app| {
			let app_handle = app.app_handle();
			tauri::async_runtime::spawn(async move {
				loop {
					sleep(Duration::from_millis(50)).await;
					// println!("sending backend-ping");
					app_handle.emit_all("backend-ping", "ping").unwrap();
				}
			});
			Ok(())
		})
		.on_page_load(|window, _| {
			let window_ = window.clone();
			println!("page loaded");
		})
		.manage(Counter::default())
		.invoke_handler(tauri::generate_handler![dom_loaded, start_game])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}

#[tauri::command]
fn dom_loaded() {
	println!("dom loaded!");
}

#[tauri::command]
async fn start_game(window: tauri::Window) -> Game {
	println!("starting game");
	let game: Game = Game::new(30, 30);
	let original_arc_mutex_game = Arc::new(Mutex::new(game.clone()));
	let cloned_game = original_arc_mutex_game.clone();
	
	tauri::async_runtime::spawn(async move {
		// let up = window.listen("direction_up", |_event| {
		// 	let mut game = cloned_game.lock().unwrap();
		// 	game.change_direction(Direction::Up)
		// });
		// let down = window.listen("direction_down", |_event| {
		// 	let mut game = cloned_game.lock().unwrap();
		// 	game.change_direction(Direction::Down)
		// });

		let mut game_lost = false;
		while !game_lost {
			sleep(Duration::from_millis(100)).await;
			let mut game = cloned_game.lock().unwrap();
			game.tick();
			window.emit("tick", game.clone()).unwrap();
			if game.lost { game_lost = true; }
		}

		// window.unlisten(up);
		// window.unlisten(down);
		window.emit("lost", "You lost the game!").unwrap();
	});
	game.clone()
}

#[tauri::command]
fn add_count(num: i32, counter: State<'_, Counter>) -> String {
	let mut val = counter.0.lock().unwrap();
	*val += num;

	format!("{val}")
}

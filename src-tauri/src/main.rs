#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]
mod snake;

use std::ptr::null;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use snake::Game;
use tauri::{Manager, State, Event, AppHandle, window};
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
	
	tauri::async_runtime::spawn(async move {
		let mut arc_game = Arc::new(game.clone());
		let up = window.listen("direction_up",  |_event|  {Arc::clone(&arc_game).change_direction(Direction::Up)});
		let down = window.listen("direction_down",  |_event| {Arc::clone(&arc_game).change_direction(Direction::Down)});
		let right = window.listen("direction_right", |_event| {Arc::clone(&arc_game).change_direction(Direction::Right)});
		let left = window.listen("direction_left", |_event| {Arc::clone(&arc_game).change_direction(Direction::Left)});
		while !arc_game.lost {
			sleep(Duration::from_millis(400)).await;
			arc_game.tick();
			// println!("sending backend-ping");
			window.emit("tick", *(arc_game.clone())).unwrap();
		}
		
		window.emit("lost", "You lost the game!").unwrap();
		window.unlisten(up);
		window.unlisten(down);
		window.unlisten(left);
		window.unlisten(right);
	});
	game.clone()
}



#[tauri::command]
fn add_count(num: i32, counter: State<'_, Counter>) -> String {
	let mut val = counter.0.lock().unwrap();
	*val += num;

	format!("{val}")
}

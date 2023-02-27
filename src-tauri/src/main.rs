#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod snake;

use snake::Game;

use std::sync::{Arc, Mutex};

use std::time::Duration;
use tauri::{Manager, State, Window, Wry};
use tokio::time::sleep;

use crate::snake::Direction;

#[derive(Default)]
struct Counter(Arc<Mutex<i32>>);

#[derive(Default)]
struct SafeGame(Arc<Mutex<Game>>);

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
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .on_page_load(|_, _| {
            println!("page loaded");
        })
        .manage(SafeGame::default())
        .invoke_handler(tauri::generate_handler![
            dom_loaded,
            start_game,
            tick,
            queue_change_dir
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn dom_loaded() {
    println!("dom loaded!");
}

#[tauri::command]
async fn start_game(safe_game: State<'_, SafeGame>) -> Result<Game, ()> {
    println!("starting game");
    let mut game_mut = safe_game.0.lock().unwrap();
    *game_mut = Game::new(30, 30);
    Ok(game_mut.clone())
}

#[tauri::command]
async fn tick(window: Window<Wry>, safe_game: State<'_, SafeGame>) -> Result<Game, ()> {
    let mut game_mut = safe_game.0.lock().unwrap();
    game_mut.tick();
    if game_mut.lost {
        window.emit("lost", "lost game").unwrap();
        Err(())
    } else {
        Ok(game_mut.clone())
    }
}

#[tauri::command]
fn queue_change_dir(direction: String, game: State<'_, SafeGame>) {
    let mut game_mut = game.0.lock().unwrap();

    let dir: Direction = match direction.as_str() {
        "up" => Direction::Up,
        "down" => Direction::Down,
        "left" => Direction::Left,
        "right" => Direction::Right,
        _ => game_mut.snake.direction,
    };
    game_mut.queue_change_direction(dir);
}

#[tauri::command]
fn add_count(num: i32, counter: State<'_, Counter>) -> String {
    let mut val = counter.0.lock().unwrap();
    *val += num;

    format!("{val}")
}

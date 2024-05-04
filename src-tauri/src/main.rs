// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::str::FromStr;

use crate::role::Role;

mod role;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(rolelist: &str) -> String {
    let mut roles = vec![];

    for line in rolelist.lines() {
        let line = line.split_once(" ");
        if let Some(line) = line {
            let (amount, role) = line;
            dbg!(amount, role);
            let role_enum = Role::from_str(role).expect("Role unable to be converted");
            roles.push((amount, role_enum));
        }
    }

    roles.sort_by(|first, second| first.1.cmp(&second.1));

    let roles = roles
        .into_iter()
        .map(|item| format!("{} {:?}", item.0, item.1))
        .collect::<Vec<String>>();

    dbg!(roles.join("\n").trim().to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

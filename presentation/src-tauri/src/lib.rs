use application::env::load_env_file;
use persistance::migrate_up;

mod commands;

// TODO: TOGGLE SOUND 
// TODO: Add 5 sec every correct answer
// TODO: Track difficulty, and generate equations based on difficulty 

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    std::panic::set_hook(Box::new(|info| {
        println!("🔥 Panic in mobile entry point: {:?}", info);
    }));

    load_env_file(cfg!(test));
    migrate_up().await.unwrap();

    let builder = tauri::Builder::default();

    builder
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_equation::get_equation,
            commands::get_settings::get_settings,
            commands::create_questioner::create_questioner,
            commands::get_stats::get_stats
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

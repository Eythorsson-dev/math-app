use serde::Serialize;

#[derive(Serialize)]
pub struct SettingsDto {
    correct_audio_src: Option<String>,
    game_duration_sec: i8,
}

pub enum CorrectAudio {
    AchievementBell,
    MaleVoiceCheer,
    MaleVoiceYes,
    QuickWin,
    UnlockGame,
}

impl CorrectAudio {
    pub fn get_src(&self) -> String {
        match self {
            CorrectAudio::AchievementBell => {
                "correct-answer/mixkit-achievement-bell-600.mp3".to_owned()
            }
            CorrectAudio::MaleVoiceCheer => {
                "correct-answer/mixkit-male-voice-cheer-2010.mp3".to_owned()
            }
            CorrectAudio::MaleVoiceYes => {
                "correct-answer/mixkit-males-yes-victory-2012.mp3".to_owned()
            }
            CorrectAudio::QuickWin => {
                "correct-answer/mixkit-quick-win-video-game-notification-269.mp3".to_owned()
            }
            CorrectAudio::UnlockGame => {
                "correct-answer/mixkit-unlock-game-notification-253.mp3".to_owned()
            }
        }
    }
}

#[tauri::command]
pub fn get_settings() -> SettingsDto {
    SettingsDto {
        correct_audio_src: Some(CorrectAudio::AchievementBell.get_src()),
        game_duration_sec: 45,
    }
}

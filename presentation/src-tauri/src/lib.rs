use domain::*;
use expression::Expression;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ExpressionDto {
    expression: Vec<String>,
    answer: f32,
}

impl From<Expression> for ExpressionDto {
    fn from(value: Expression) -> Self {
        ExpressionDto {
            expression: value.formatted_vec(),
            answer: value.get_answer(),
        }
    }
}

#[tauri::command]
fn get_equation() -> ExpressionDto {
    let options = ExpressionOption {
        constant: ConstantOption::new(1, 10).unwrap(),
        allowed_operators: AllowedOperators::new(vec![
            Operator::Addition,
            Operator::Subtraction,
            Operator::Multiplication,
            Operator::Division,
        ])
        .unwrap(),
        term_count: TermCount::new(3).unwrap(),
    };

    let weights = OperatorWeights::new(vec![]);

    let equation = Equation::generate(&options, weights).unwrap();

    equation.expression.into()
}

#[derive(Serialize)]
struct SettingsDto {
    correct_audio_src: Option<String>,
    game_duration_sec: i8
}

#[tauri::command]
fn get_settings() -> SettingsDto {
    SettingsDto {
        correct_audio_src: Some(CorrectAudio::achievement_bell.get_src()),
        game_duration_sec: 45
    }
}

pub enum CorrectAudio {
    achievement_bell,
    male_voice_cheer,
    male_voice_yes,
    quick_win,
    unlock_game,
}

impl CorrectAudio {
    pub fn get_src(&self) -> String {
        match self {
            CorrectAudio::achievement_bell => {
                "correct-answer/mixkit-achievement-bell-600.mp3".to_owned()
            }
            CorrectAudio::male_voice_cheer => {
                "correct-answer/mixkit-male-voice-cheer-2010.mp3".to_owned()
            }
            CorrectAudio::male_voice_yes => {
                "correct-answer/mixkit-males-yes-victory-2012.mp3".to_owned()
            }
            CorrectAudio::quick_win => {
                "correct-answer/mixkit-quick-win-video-game-notification-269.mp3".to_owned()
            }
            CorrectAudio::unlock_game => {
                "correct-answer/mixkit-unlock-game-notification-253.mp3".to_owned()
            }
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_equation, get_settings])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

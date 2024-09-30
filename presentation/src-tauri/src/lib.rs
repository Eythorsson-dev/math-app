use application::{
    env::load_env_file, QuestionerCommand, QuestionerRepository, QuestionerStats, TaskDto,
};
use domain::*;
use expression::{
    operator::Operator,
    operator_weights::OperatorWeights,
    options::{AllowedOperators, ConstantOption, ExpressionOption, TermCount},
    Expression,
};
use persistance::{get_unit_of_work, migrate_up};
use questioner::QuestionerId;
use serde::{Deserialize, Serialize};

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

    let expression = Expression::generate(&options, weights).unwrap();

    expression.into()
}

#[derive(Serialize)]
struct SettingsDto {
    correct_audio_src: Option<String>,
    game_duration_sec: i8,
}

#[tauri::command]
fn get_settings() -> SettingsDto {
    SettingsDto {
        correct_audio_src: Some(CorrectAudio::AchievementBell.get_src()),
        game_duration_sec: 45,
    }
}

////////////////////////////////////////////////////////////////////////////////////////
#[derive(Deserialize)]
struct CreateQuestionerDto {
    id: QuestionerId,
    allotted_time: Duration,
    tasks: Vec<TaskDto>,
}

impl From<CreateQuestionerDto> for QuestionerCommand {
    fn from(value: CreateQuestionerDto) -> Self {
        QuestionerCommand::Create {
            id: value.id,
            allotted_time: value.allotted_time,
            tasks: value.tasks,
        }
    }
}

#[tauri::command]
async fn create_questioner(request: CreateQuestionerDto) {
    // TODO: REMOVE UNWRAP
    let uow = get_unit_of_work(cfg!(test)).await.unwrap();
    let command: QuestionerCommand = request.into();

    command.handle(&uow).await.unwrap();
}

#[cfg(test)]
mod tests {
    use crate::CreateQuestionerDto;

    #[test]
    pub fn can_deserialize() {
        let json_str = r#"
        {
            "id": "6b69f438-e9cf-4e4b-94f9-9338fb79f805",
            "allotted_time": 45,
            "tasks": [
                {
                    "expression": "9*(9+5)",
                    "answered": 127,
                    "answer_correct": false,
                    "answer_duration": 11,
                    "answered_at": 1727628637
                }
            ]
        }
        "#;

        let _value: CreateQuestionerDto = serde_json::from_str(json_str).unwrap();
    }
}

////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize)]
struct QuestionerStatsDto {
    pub high_score: u32,
    pub daily_streak: u32,
    pub previous_score: u32,
}

impl From<QuestionerStats> for QuestionerStatsDto {
    fn from(value: QuestionerStats) -> Self {
        Self {
            high_score: value.high_score,
            daily_streak: value.daily_streak,
            previous_score: value.previous_score,
        }
    }
}

#[tauri::command]
async fn get_stats() -> QuestionerStatsDto {
    // TODO: REMOVE UNWRAP
    let uow = get_unit_of_work(cfg!(test)).await.unwrap();

    let stats = uow.questioner.get_stats().await.unwrap();

    stats.into()
}

////////////////////////////////////////////////////////////////////////////////////////

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    load_env_file(cfg!(test));

    migrate_up().await.unwrap();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_equation,
            get_settings,
            create_questioner,
            get_stats
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

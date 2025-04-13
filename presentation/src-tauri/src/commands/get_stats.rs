use application::QuestionerRepository;
use application::QuestionerStats;
use persistance::get_unit_of_work;
use serde::Serialize;

#[derive(Serialize)]
pub struct QuestionerStatsDto {
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
pub async fn get_stats() -> QuestionerStatsDto {
    // TODO: REMOVE UNWRAP
    let uow = get_unit_of_work(cfg!(test)).await.unwrap();

    let stats = uow.questioner.get_stats().await.unwrap();

    stats.into()
}

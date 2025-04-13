use application::{QuestionerCommand, TaskDto};
use domain::{questioner::QuestionerId, Duration};
use persistance::get_unit_of_work;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateQuestionerDto {
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
pub async fn create_questioner(request: CreateQuestionerDto) {
    // TODO: REMOVE UNWRAP
    let uow = get_unit_of_work(cfg!(test)).await.unwrap();
    let command: QuestionerCommand = request.into();

    command.handle(&uow).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::CreateQuestionerDto;

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

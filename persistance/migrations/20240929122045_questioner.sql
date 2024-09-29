-- Add migration script here

CREATE TABLE Questioners (
    id TEXT NOT NULL PRIMARY KEY,
    allotted_time BIGINT NOT NULL,
    correct_answers INTEGER NOT NULL
);

CREATE TABLE Tasks (
    questioner_id TEXT NOT NULL,
    expression TEXT NOT NULL,

    answered INTEGER NOT NULL,
    answer_correct BOOLEAN NOT NULL,
    answer_duration BIGINT NOT NULL,
    answered_at BIGINT NOT NULL,

    FOREIGN KEY(questioner_id) REFERENCES Questioners(id)
);
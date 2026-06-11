CREATE TABLE IF NOT EXISTS exercise (
    id            TEXT PRIMARY KEY,
    instructions  TEXT NOT NULL,
    name          TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS plan (
    id          TEXT PRIMARY KEY, 
    name        TEXT NOT NULL,
    description TEXT NOT NULL,
    est_mins    INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS plan_exercise (
    plan_id     INTEGER NOT NULL,
    exercise_id INTEGER NOT NULL,
    order_index INTEGER NOT NULL,
    reps        INTEGER,
    PRIMARY KEY (plan_id, exercise_id),
    FOREIGN KEY (plan_id)     REFERENCES workout_plan(id),
    FOREIGN KEY (exercise_id) REFERENCES exercise(id)
);

CREATE TABLE IF NOT EXISTS completed_workout (
    id          TEXT PRIMARY KEY,
    plan_id     TEXT,
    date        TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    time_min    INTEGER NOT NULL,
    FOREIGN KEY (plan_id) REFERENCES plan(id)
);

CREATE TABLE IF NOT EXISTS mahlzeit (
    name        TEXT PRIMARY KEY,
    kalorien    INTEGER NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS zutat (
    name    TEXT PRIMARY KEY
);

create TABLE IF NOT EXISTS zutat_in_mahlzeit (
    anzahl        INTEGER NOT NULL,
    mahlzeit_name TEXT NOT NULL,
    zutat_name    TEXT NOT NULL,
    PRIMARY KEY   (mahlzeit_name, zutat_name),
    FOREIGN KEY   (mahlzeit_name) REFERENCES mahlzeit(name),
    FOREIGN KEY   (zutat_name)    REFERENCES zutat(name)
);

create TABLE IF NOT EXISTS mahlzeit_gemampft (
    date  TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    mahlzeit_name  TEXT NOT NULL, 
    FOREIGN KEY (mahlzeit_name) REFERENCES mahlzeit(name)
)

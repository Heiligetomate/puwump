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
    id          TEXT PRIMARY KEY,
    plan_id     TEXT NOT NULL,
    exercise_id TEXT NOT NULL,
    order_index INTEGER NOT NULL,
    reps        INTEGER,
    FOREIGN KEY (plan_id)     REFERENCES plan(id)     ON DELETE CASCADE,
    FOREIGN KEY (exercise_id) REFERENCES exercise(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS completed_workout (
    id          TEXT PRIMARY KEY,
    plan_id     TEXT,
    date        TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    time_min    INTEGER NOT NULL,
    FOREIGN KEY (plan_id) REFERENCES plan(id)
);

CREATE TABLE IF NOT EXISTS meal (
    id          TEXT PRIMARY KEY, 
    name        TEXT NOT NULL UNIQUE,
    calories    INTEGER NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS ingredient (
    id    TEXT PRIMARY KEY,
    name  TEXT NOT NULL UNIQUE
);

create TABLE IF NOT EXISTS ingredient_in_meal (
    id              TEXT PRIMARY KEY,
    amount_gr       INTEGER NOT NULL,
    meal_name       TEXT NOT NULL,
    ingredient_id   TEXT NOT NULL,
    FOREIGN KEY     (meal_id)       REFERENCES meal(id)       ON DELETE CASCADE, 
    FOREIGN KEY     (ingredient_id) REFERENCES ingredient(id) ON DELETE CASCADE
);

create TABLE IF NOT EXISTS meal_inhaled (
    id          TEXT PRIMARY KEY, 
    date        TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    meal_name   TEXT NOT NULL, 
    FOREIGN KEY (meal_name) REFERENCES meal(name)
)

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

CREATE TABLE IF NOT EXISTS meal (
    name        TEXT PRIMARY KEY,
    calories    INTEGER NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS ingredient (
    name    TEXT PRIMARY KEY
);

create TABLE IF NOT EXISTS ingredient_in_meal (
    amount_gr       INTEGER NOT NULL,
    meal_name       TEXT NOT NULL,
    ingredient_name TEXT NOT NULL,
    PRIMARY KEY     (meal_name, ingredient_name),
    FOREIGN KEY     (meal_name) REFERENCES meal(name),
    FOREIGN KEY     (ingredient_name) REFERENCES ingredient(name)
);

create TABLE IF NOT EXISTS meal_inhaled (
    date        TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    meal_name   TEXT NOT NULL, 
    FOREIGN KEY (meal_name) REFERENCES meal(name)
)

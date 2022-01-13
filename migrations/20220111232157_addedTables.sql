-- Add migration script here
CREATE TABLE IF NOT EXISTS Project (
    id BLOB NOT NULL PRIMARY KEY,
    name VARCHAR(32) NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS Tasks (
    id BLOB NOT NULL PRIMARY KEY,
    name VARCHAR(32) NOT NULL,
    description TEXT NOT NULL,
    project_id BLOB NOT NULL REFERENCES Project(id),
    progress REAL NOT NULL CHECK (progress >= 0 AND progress <= 1)
);

CREATE TABLE IF NOT EXISTS Departments (
    id BLOB NOT NULL PRIMARY KEY,
    name VARCHAR(32) NOT NULL
);

CREATE TABLE IF NOT EXISTS Workers (
    id BLOB NOT NULL PRIMARY KEY,
    name VARCHAR(32) NOT NULL,
    last_name VARCHAR(64) NOT NULL,
    department_id BLOB NOT NULL REFERENCES Departments(id),
    created_time DATETIME NOT NULL,
    role INT NOT NULL
);

CREATE TABLE IF NOT EXISTS Assignments (
    worker_id BLOB NOT NULL REFERENCES Workers(id),
    task_id BLOB NOT NULL REFERENCES Tasks(id),

    UNIQUE(worker_id, task_id)
);
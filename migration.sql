CREATE TABLE todos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    description TEXT NOT NULL,
    creation_date DATETIME,
    due_date DATETIME,
    starred INTEGER DEFAULT 0,
    is_completed BOOLEAN DEFAULT 0
);
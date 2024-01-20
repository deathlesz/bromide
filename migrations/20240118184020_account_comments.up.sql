CREATE TABLE "account_comments" (
    id INTEGER NOT NULL UNIQUE,
    text TEXT NOT NULL,
    user_id INTEGER NOT NULL,
    likes INTEGER NOT NULL DEFAULT 0,
    dislikes INTEGER NOT NULL DEFAULT 0,
    is_spam INTEGER NOT NULL DEFAULT 0,
    timestamp INTEGER NOT NULL DEFAULT (UNIXEPOCH('now')),

    PRIMARY KEY ("id" AUTOINCREMENT),
    FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE
)
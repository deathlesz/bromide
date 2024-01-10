DROP TABLE IF EXISTS `accounts`;
CREATE TABLE "accounts"
(
    "id"        INTEGER NOT NULL UNIQUE,
    "user_name" TEXT    NOT NULL UNIQUE COLLATE NOCASE,
    "email"     TEXT    NOT NULL UNIQUE COLLATE NOCASE,
    "password"  TEXT    NOT NULL,

    PRIMARY KEY ("id" AUTOINCREMENT)
);

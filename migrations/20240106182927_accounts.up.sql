DROP TABLE IF EXISTS `accounts`;
CREATE TABLE "accounts" (
    "id" INTEGER,
    "user_name" TEXT NOT NULL UNIQUE,
    "email" TEXT NOT NULL UNIQUE,
    "password" TEXT NOT NULL,

    PRIMARY KEY("id" AUTOINCREMENT)
);

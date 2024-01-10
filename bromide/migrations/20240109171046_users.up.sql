DROP TABLE IF EXISTS `users`;
CREATE TABLE "users"
(
    "id"              INTEGER NOT NULL UNIQUE,
    "stars"           INTEGER NOT NULL DEFAULT 0,
    "moons"           INTEGER NOT NULL DEFAULT 0,
    "diamonds"        INTEGER NOT NULL DEFAULT 0,
    "secret_coins"    INTEGER NOT NULL DEFAULT 0,
    "user_coins"      INTEGER NOT NULL DEFAULT 0,
    "demons"          INTEGER NOT NULL DEFAULT 0,
    "creator_points"  INTEGER NOT NULL DEFAULT 0,
    "icon_id"         INTEGER NOT NULL DEFAULT 1,
    "icon_type"       INTEGER NOT NULL DEFAULT 0,
    "glowing"         INTEGER NOT NULL DEFAULT 0,
    "primary_color"   INTEGER NOT NULL DEFAULT 0,
    "secondary_color" INTEGER NOT NULL DEFAULT 3,
    "glow_color"      INTEGER NOT NULL DEFAULT 0,
    "cube_id"         INTEGER NOT NULL DEFAULT 1,
    "ship_id"         INTEGER NOT NULL DEFAULT 1,
    "ball_id"         INTEGER NOT NULL DEFAULT 1,
    "ufo_id"          INTEGER NOT NULL DEFAULT 1,
    "wave_id"         INTEGER NOT NULL DEFAULT 1,
    "robot_id"        INTEGER NOT NULL DEFAULT 1,
    "spider_id"       INTEGER NOT NULL DEFAULT 1,
    "swing_id"        INTEGER NOT NULL DEFAULT 1,
    "jetpack_id"      INTEGER NOT NULL DEFAULT 1,
    "glow_id"         INTEGER NOT NULL DEFAULT 0,
    "explosion_id"    INTEGER NOT NULL DEFAULT 1,
    "account_id"      INTEGER NOT NULL UNIQUE,

    PRIMARY KEY ("id" AUTOINCREMENT),
    FOREIGN KEY ("account_id") REFERENCES "accounts" ("id") ON DELETE CASCADE
);
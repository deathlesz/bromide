CREATE TABLE "users"
(
    "id"                        INTEGER NOT NULL UNIQUE,
    "user_name"                 TEXT    NOT NULL UNIQUE COLLATE NOCASE,
    "email"                     TEXT    NOT NULL UNIQUE COLLATE NOCASE,
    "password"                  TEXT    NOT NULL,

    "icon_id"                   INTEGER NOT NULL DEFAULT 1,
    "icon_type"                 INTEGER NOT NULL DEFAULT 0,
    "glowing"                   INTEGER NOT NULL DEFAULT 0,
    "primary_color"             INTEGER NOT NULL DEFAULT 0,
    "secondary_color"           INTEGER NOT NULL DEFAULT 3,
    "tertiary_color"            INTEGER NOT NULL DEFAULT 0,
    "cube_id"                   INTEGER NOT NULL DEFAULT 1,
    "ship_id"                   INTEGER NOT NULL DEFAULT 1,
    "ball_id"                   INTEGER NOT NULL DEFAULT 1,
    "ufo_id"                    INTEGER NOT NULL DEFAULT 1,
    "wave_id"                   INTEGER NOT NULL DEFAULT 1,
    "robot_id"                  INTEGER NOT NULL DEFAULT 1,
    "spider_id"                 INTEGER NOT NULL DEFAULT 1,
    "swing_id"                  INTEGER NOT NULL DEFAULT 1,
    "jetpack_id"                INTEGER NOT NULL DEFAULT 1,
    "glow_id"                   INTEGER NOT NULL DEFAULT 0,
    "explosion_id"              INTEGER NOT NULL DEFAULT 1,

    "stars"                     INTEGER NOT NULL DEFAULT 0,
    "moons"                     INTEGER NOT NULL DEFAULT 0,
    "diamonds"                  INTEGER NOT NULL DEFAULT 0,
    "secret_coins"              INTEGER NOT NULL DEFAULT 0,
    "user_coins"                INTEGER NOT NULL DEFAULT 0,
    "creator_points"            INTEGER NOT NULL DEFAULT 0,

    "easy_normal_demons"        INTEGER NOT NULL DEFAULT 0,
    "medium_normal_demons"      INTEGER NOT NULL DEFAULT 0,
    "hard_normal_demons"        INTEGER NOT NULL DEFAULT 0,
    "insane_normal_demons"      INTEGER NOT NULL DEFAULT 0,
    "extreme_normal_demons"     INTEGER NOT NULL DEFAULT 0,
    "easy_platformer_demons"    INTEGER NOT NULL DEFAULT 0,
    "medium_platformer_demons"  INTEGER NOT NULL DEFAULT 0,
    "hard_platformer_demons"    INTEGER NOT NULL DEFAULT 0,
    "insane_platformer_demons"  INTEGER NOT NULL DEFAULT 0,
    "extreme_platformer_demons" INTEGER NOT NULL DEFAULT 0,
    "weekly_demons"             INTEGER NOT NULL DEFAULT 0,
    "gauntlet_demons"           INTEGER NOT NULL DEFAULT 0,

    "message_state"             INTEGER NOT NULL DEFAULT 0,
    "friend_state"              INTEGER NOT NULL DEFAULT 0,
    "comment_history_state"     INTEGER NOT NULL DEFAULT 0,

    "youtube_url"               TEXT    NOT NULL DEFAULT '',
    "twitter_url"               TEXT    NOT NULL DEFAULT '',
    "twitch_url"                TEXT    NOT NULL DEFAULT '',

    "mod_level"                 INTEGER NOT NULL DEFAULT 0,

    PRIMARY KEY ("id" AUTOINCREMENT)
);
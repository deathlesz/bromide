DROP TABLE IF EXISTS users;

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    account_id INT UNIQUE NOT NULL,

    icon_id INT NOT NULL DEFAULT 1,
    icon_type INT NOT NULL DEFAULT 0,
    glowing INT NOT NULL DEFAULT 0,
    primary_color INT NOT NULL DEFAULT 0,
    secondary_color INT NOT NULL DEFAULT 3,
    tertiary_color INT NOT NULL DEFAULT 0,
    cube_id INT NOT NULL DEFAULT 1,
    ship_id INT NOT NULL DEFAULT 1,
    ball_id INT NOT NULL DEFAULT 1,
    ufo_id INT NOT NULL DEFAULT 1,
    wave_id INT NOT NULL DEFAULT 1,
    robot_id INT NOT NULL DEFAULT 1,
    spider_id INT NOT NULL DEFAULT 1,
    swing_id INT NOT NULL DEFAULT 1,
    jetpack_id INT NOT NULL DEFAULT 1,
    glow_id INT NOT NULL DEFAULT 0,
    explosion_id INT NOT NULL DEFAULT 1,

    stars INT NOT NULL DEFAULT 0,
    moons INT NOT NULL DEFAULT 0,
    diamonds INT NOT NULL DEFAULT 0,
    secret_coins INT NOT NULL DEFAULT 0,
    user_coins INT NOT NULL DEFAULT 0,
    creator_points INT NOT NULL DEFAULT 0,

    message_state INT NOT NULL DEFAULT 0,
    friend_state INT NOT NULL DEFAULT 0,
    comment_history_state INT NOT NULL DEFAULT 0,

    youtube_url TEXT NOT NULL DEFAULT '',
    twitter_url TEXT NOT NULL DEFAULT '',
    twitch_url TEXT NOT NULL DEFAULT '',

    mod_level INT NOT NULL DEFAULT 0,

    CONSTRAINT users_account_id_fkey FOREIGN KEY (account_id) REFERENCES accounts (id) ON DELETE CASCADE
)

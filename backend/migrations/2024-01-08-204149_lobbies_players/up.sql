CREATE TABLE lobbies_players
(
    "lobby_id" CHAR(10) NOT NULL,
    "player_id" VARCHAR(100) NOT NULL,
    "contents_id" UUID NULL DEFAULT NULL,
    "is_ready" BOOLEAN NOT NULL DEFAULT FALSE,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "lobbies_players_pkey" PRIMARY KEY ("lobby_id", "player_id"),
    CONSTRAINT "player_id" FOREIGN KEY ("player_id") REFERENCES users ("id")
        ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT "lobby_id" FOREIGN KEY ("lobby_id") REFERENCES lobbies ("id")
        ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT "contents_id" FOREIGN KEY ("contents_id") REFERENCES contents ("id")
        ON DELETE NO ACTION ON UPDATE NO ACTION
);

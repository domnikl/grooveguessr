CREATE TABLE contents
(
    "lobby_id" CHAR(10) NOT NULL,
    "user_id" VARCHAR(100) NOT NULL,
    "type" VARCHAR(70) NOT NULL,
    "data" VARCHAR(255) NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "contents_pkey" PRIMARY KEY ("lobby_id", "user_id"),
    CONSTRAINT "contents_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES users ("id")
        ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT "contents_lobby_id_fkey" FOREIGN KEY ("lobby_id") REFERENCES lobbies ("id")
        ON DELETE CASCADE ON UPDATE CASCADE
);

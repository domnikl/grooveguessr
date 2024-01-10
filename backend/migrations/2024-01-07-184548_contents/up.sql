CREATE TABLE contents
(
    "id" UUID NOT NULL,
    "type" VARCHAR(70) NOT NULL,
    "data" VARCHAR(255) NOT NULL,
    "user_id" VARCHAR(100) NOT NULL,
    "lobby_id" CHAR(10) NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "contents_pkey" PRIMARY KEY ("id"),
    CONSTRAINT "contents_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES users ("id")
        ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT "contents_lobby_id_fkey" FOREIGN KEY ("lobby_id") REFERENCES lobbies ("id")
        ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT "contents_user_lobby_unique" UNIQUE ("user_id", "lobby_id")
);

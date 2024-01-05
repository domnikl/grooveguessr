CREATE TABLE lobbies
(
    "id" CHAR(10) NOT NULL,
    "started_at" TIMESTAMPTZ NULL DEFAULT NULL,
    "guessing_time" SMALLINT NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "lobbies_pkey" PRIMARY KEY ("id")
);

COMMENT ON COLUMN lobbies.guessing_time IS 'in seconds';

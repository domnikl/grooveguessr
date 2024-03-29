CREATE TABLE lobbies
(
    "id" CHAR(10) NOT NULL,
    "started_at" TIMESTAMPTZ NULL DEFAULT NULL,
    "guessing_time" SMALLINT NOT NULL,
    "host_id" VARCHAR(100) NOT NULL,
    "sequence" TEXT NULL DEFAULT NULL,
    "current_user_id" VARCHAR(100) NULL DEFAULT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "lobbies_pkey" PRIMARY KEY ("id"),
    CONSTRAINT "lobbies_host_id_fkey" FOREIGN KEY ("host_id") REFERENCES users ("id")
        ON DELETE CASCADE ON UPDATE CASCADE
);

COMMENT ON COLUMN lobbies.guessing_time IS 'in seconds';

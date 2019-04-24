CREATE TABLE urls (
  id             UUID         PRIMARY KEY,
  user_id        UUID         REFERENCES users(id) ON DELETE CASCADE,
  slug           VARCHAR(100) NOT NULL CONSTRAINT "Short URL" UNIQUE,
  url            VARCHAR(256) NOT NULL,
  visits         BIGINT       NOT NULL DEFAULT 0,
  created_at     TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
  updated_at     TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now()
);

SELECT diesel_manage_updated_at('urls');

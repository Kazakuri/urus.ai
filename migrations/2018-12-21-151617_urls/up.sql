CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE urls (
  id             UUID         PRIMARY KEY DEFAULT uuid_generate_v4(),
  slug           VARCHAR(100) NOT NULL CONSTRAINT "Short URL" UNIQUE,
  url            VARCHAR      NOT NULL,
  visits         BIGINT       NOT NULL DEFAULT 0,
  created_at     TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
  updated_at     TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now()
);

SELECT diesel_manage_updated_at('urls');

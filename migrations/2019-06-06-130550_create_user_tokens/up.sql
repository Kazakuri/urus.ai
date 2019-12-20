CREATE TYPE "TokenScope" AS ENUM ('activation');

CREATE TABLE user_tokens (
  id             UUID         PRIMARY KEY DEFAULT uuid_generate_v4(),
  user_id        UUID         NOT NULL,
  scope          "TokenScope"   NOT NULL,
  created_at     TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
  updated_at     TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now()
);

SELECT diesel_manage_updated_at('user_tokens');

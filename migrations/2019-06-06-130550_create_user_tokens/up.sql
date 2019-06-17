CREATE TYPE token_scope AS ENUM ('activation');

CREATE TABLE user_tokens (
  id             UUID         PRIMARY KEY,
  user_id        UUID         NOT NULL,
  scope          token_scope   NOT NULL,
  created_at     TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
  updated_at     TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now()
);

SELECT diesel_manage_updated_at('user_tokens');

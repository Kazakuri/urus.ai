CREATE DOMAIN EMAIL AS TEXT
  CONSTRAINT "Email" CHECK ( value ~ '^[a-zA-Z0-9.!#$%&''*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$' );

CREATE DOMAIN USERNAME AS VARCHAR(100)
  CONSTRAINT "Display Name" CHECK ( value ~ '^[a-zA-Z][-._a-zA-Z0-9]*$' );

CREATE TABLE users (
  id             UUID         PRIMARY KEY,
  display_name   USERNAME     NOT NULL CONSTRAINT "Display Name" UNIQUE,
  email          EMAIL        NOT NULL CONSTRAINT "Email" UNIQUE,
  email_verified BOOLEAN      NOT NULL DEFAULT 'f',
  password_hash  VARCHAR(128) NOT NULL,
  created_at     TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
  updated_at     TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now()
);

SELECT diesel_manage_updated_at('users');

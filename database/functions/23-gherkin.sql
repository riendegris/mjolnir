SET CLIENT_MIN_MESSAGES TO INFO;
SET CLIENT_ENCODING = 'UTF8';
-- DROP SCHEMA IF EXISTS main CASCADE;
-- CREATE SCHEMA main AUTHORIZATION odin;
-- GRANT ALL ON SCHEMA main TO odin;
-- SET SEARCH_PATH = main;

CREATE TABLE main.features (
  id UUID PRIMARY KEY DEFAULT public.gen_random_uuid(),
  title VARCHAR(256) NOT NULL,
  description TEXT,
  tags TEXT[] DEFAULT '{}',
  search TSVECTOR GENERATED ALWAYS AS (
    (
      setweight(to_tsvector('english', public.textarr2text(tags)), 'A') || ' ' ||
      setweight(to_tsvector('english', title), 'B') || ' ' ||
      setweight(to_tsvector('english', description), 'C')
    )::tsvector
  ) STORED,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE main.features OWNER TO odin;

CREATE TABLE main.scenarios (
  id UUID PRIMARY KEY DEFAULT public.gen_random_uuid(),
  feature UUID REFERENCES main.features(id),
  title VARCHAR(256) NOT NULL,
  description TEXT,
  tags TEXT[] DEFAULT '{}',
  search TSVECTOR GENERATED ALWAYS AS (
    (
      setweight(to_tsvector('english', public.textarr2text(tags)), 'A') || ' ' ||
      setweight(to_tsvector('english', title), 'B') || ' ' ||
      setweight(to_tsvector('english', description), 'C')
    )::tsvector
  ) STORED,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE main.scenarios OWNER TO odin;


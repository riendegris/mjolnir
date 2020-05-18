SET CLIENT_MIN_MESSAGES TO INFO;
SET CLIENT_ENCODING = 'UTF8';

CREATE TYPE main.step_type AS ENUM ('given', 'when', 'then');

CREATE TABLE main.features (
  id UUID PRIMARY KEY DEFAULT public.gen_random_uuid(),
  name VARCHAR(256) NOT NULL,
  description TEXT,
  tags TEXT[] DEFAULT '{}',
  search TSVECTOR GENERATED ALWAYS AS (
    (
      setweight(to_tsvector('english', public.textarr2text(tags)), 'A') || ' ' ||
      setweight(to_tsvector('english', name), 'B') || ' ' ||
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
  name VARCHAR(256) NOT NULL,
  tags TEXT[] DEFAULT '{}',
  search TSVECTOR GENERATED ALWAYS AS (
    (
      setweight(to_tsvector('english', public.textarr2text(tags)), 'A') || ' ' ||
      setweight(to_tsvector('english', name), 'B')
    )::tsvector
  ) STORED,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE main.scenarios OWNER TO odin;

CREATE TABLE main.backgrounds (
  id UUID PRIMARY KEY DEFAULT public.gen_random_uuid(),
  feature UUID REFERENCES main.features(id),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE main.backgrounds OWNER TO odin;

CREATE TABLE main.steps (
  id UUID PRIMARY KEY DEFAULT public.gen_random_uuid(),
  step_type step_type,
  value VARCHAR(256),
  docstring VARCHAR(256),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE main.steps OWNER TO odin;

CREATE TABLE main.background_step_map (
  background UUID REFERENCES main.backgrounds(id),
  step UUID REFERENCES main.steps(id),
  PRIMARY KEY (background, step)
);

ALTER TABLE main.background_step_map OWNER TO odin;

CREATE TABLE main.scenario_step_map (
  scenario UUID REFERENCES main.scenarios(id),
  step UUID REFERENCES main.steps(id),
  PRIMARY KEY (scenario, step)
);

ALTER TABLE main.scenario_step_map OWNER TO odin;

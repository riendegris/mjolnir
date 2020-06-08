CREATE TYPE main.index_status AS ENUM (
    'not_available'
  , 'download_in_progress'
  , 'download_error'
  , 'downloaded'
  , 'indexing_in_progress'
  , 'indexing_error'
  , 'indexed'
  , 'validation_in_progress'
  , 'validation_error'
  , 'available');

CREATE TABLE main.index_types (
  id VARCHAR(256) PRIMARY KEY
);

ALTER TABLE main.index_types OWNER TO odin;

CREATE TABLE main.data_sources (
  id VARCHAR(256) PRIMARY KEY
);

ALTER TABLE main.data_sources OWNER TO odin;

CREATE TABLE main.index_type_data_source (
  index_type VARCHAR(256) REFERENCES main.index_types(id) ON DELETE CASCADE,
  data_source VARCHAR(256) REFERENCES main.data_sources(id) ON DELETE CASCADE
);

ALTER TABLE main.index_type_data_source OWNER TO odin;

CREATE TABLE main.environments (
  id UUID PRIMARY KEY DEFAULT public.gen_random_uuid(),
  signature TEXT CONSTRAINT unique_environment_signature UNIQUE DEFAULT public.random_signature(),
  status main.index_status NOT NULL DEFAULT 'not_available',
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE main.environments OWNER TO odin;

CREATE TRIGGER notify_environments
AFTER INSERT OR UPDATE
ON main.environments
FOR EACH ROW
  EXECUTE PROCEDURE main.tg_notify('notifications');


CREATE TABLE main.scenario_environment_map (
  scenario UUID REFERENCES main.scenarios(id) ON DELETE CASCADE,
  environment UUID REFERENCES main.environments(id) ON DELETE CASCADE,
  PRIMARY KEY (scenario, environment)
);

ALTER TABLE main.scenario_environment_map OWNER TO odin;

CREATE TABLE main.background_environment_map (
  background UUID REFERENCES main.backgrounds(id) ON DELETE CASCADE,
  environment UUID REFERENCES main.environments(id) ON DELETE CASCADE,
  PRIMARY KEY (background, environment)
);

ALTER TABLE main.background_environment_map OWNER TO odin;

CREATE TABLE main.indexes (
  id UUID PRIMARY KEY DEFAULT public.gen_random_uuid(),
  index_type VARCHAR(32) REFERENCES main.index_types(id) ON DELETE CASCADE,
  data_source VARCHAR(32) REFERENCES main.data_sources(id) ON DELETE CASCADE,
  regions TEXT[],
  signature TEXT
    GENERATED ALWAYS AS (MD5(index_type || '-' || data_source || '-' || public.array2string(regions))) STORED
    CONSTRAINT unique_index_signature UNIQUE,
  filepath VARCHAR(256),
  status main.index_status NOT NULL DEFAULT 'not_available',
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  -- FIXME This constraints is used to make sure that the data source is compatible with the index type.
  -- FOREIGN KEY (index_type, data_source) REFERENCES main.index_type_data_source (index_type, data_source)
  -- TODO The following is redundant with the unique_index_signature, so maybe remove it.
  -- Test with and without...
  UNIQUE (index_type, data_source, regions)
);

ALTER TABLE main.indexes OWNER TO odin;

CREATE TABLE main.environment_index_map (
  environment UUID REFERENCES main.environments(id) ON DELETE CASCADE,
  index_id UUID REFERENCES main.indexes(id) ON DELETE CASCADE,
  PRIMARY KEY (environment, index_id)
);

ALTER TABLE main.environment_index_map OWNER TO odin;



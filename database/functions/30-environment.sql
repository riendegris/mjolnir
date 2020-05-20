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
  signature VARCHAR(256) NOT NULL DEFAULT '',
  status main.index_status NOT NULL DEFAULT 'not_available',
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE main.environments OWNER TO odin;

CREATE TABLE main.scenario_environment_map (
  scenario UUID REFERENCES main.scenarios(id) ON DELETE CASCADE,
  environment UUID REFERENCES main.environments(id) ON DELETE CASCADE,
  PRIMARY KEY (scenario, environment)
);

ALTER TABLE main.scenario_environment_map OWNER TO odin;

CREATE TABLE main.indexes (
  id UUID PRIMARY KEY DEFAULT public.gen_random_uuid(),
  signature VARCHAR(256) UNIQUE NOT NULL,
  index_type VARCHAR(256) REFERENCES main.index_types(id) ON DELETE CASCADE,
  data_source VARCHAR(256) REFERENCES main.data_sources(id) ON DELETE CASCADE,
  region VARCHAR(256),
  filepath VARCHAR(256),
  status main.index_status NOT NULL DEFAULT 'not_available',
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  -- FIXME This constraints is used to make sure that the data source is compatible with the index type.
  -- FOREIGN KEY (index_type, data_source) REFERENCES main.index_type_data_source (index_type, data_source)
  UNIQUE (index_type, data_source, region)
);

ALTER TABLE main.indexes OWNER TO odin;


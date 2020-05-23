-- This type is used to return a environment to the client
CREATE TYPE main.return_environment_type AS (
    id          UUID
  , signature   TEXT
  , status      main.index_status
  , created_at  TIMESTAMPTZ
  , updated_at  TIMESTAMPTZ
);

-- This type is used to return an index to the client
CREATE TYPE main.return_index_type AS (
    id          UUID
  , signature   TEXT
  , index_type  TEXT
  , data_source TEXT
  , regions      TEXT[]
  , filepath    TEXT
  , status      main.index_status
  , created_at  TIMESTAMPTZ
  , updated_at  TIMESTAMPTZ
);

-- It's unlikely we'll call this function, because we don't explicitely
-- create test environments. When we create an index, attached to a scenario,
-- we check if there is an environment, and if not, we create one.
CREATE OR REPLACE FUNCTION main.create_or_replace_environment (
    _id          UUID                 -- id          (1)
  , _status      main.index_status    -- status      (3)
) RETURNS main.return_environment_type
AS $$
DECLARE
  res main.return_environment_type;
BEGIN
  INSERT INTO main.environments VALUES (
      $1  -- id
    , $2  -- status
  )
  ON CONFLICT (id) DO
    UPDATE
    SET   status = EXCLUDED.status
        , updated_at = NOW();
  SELECT id, signature, status, created_at, updated_at FROM main.environments WHERE id = $1 INTO res;
  RETURN res;
END;
$$
LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION main.create_or_replace_index (
    _index_type  TEXT     -- (1)
  , _data_source TEXT     -- (2)
  , _regions      TEXT[]  -- (3)
) RETURNS main.return_index_type
AS $$
DECLARE
  res main.return_index_type;
  v_state   TEXT;
  v_msg     TEXT;
  v_detail  TEXT;
  v_hint    TEXT;
  v_context TEXT;
BEGIN
  INSERT INTO main.indexes (index_type, data_source, regions) VALUES (
      $1  -- index type
    , $2  -- data source
    , $3  -- regions
  )
  ON CONFLICT ON CONSTRAINT unique_index_signature DO
    UPDATE
    SET   index_type  = EXCLUDED.index_type
        , data_source = EXCLUDED.data_source
        , regions     = EXCLUDED.regions
        , updated_at  = NOW()
  RETURNING id, signature, index_type, data_source, regions, filepath, status, created_at, updated_at INTO res;
  RETURN res;
  EXCEPTION
  WHEN others
  THEN
    GET STACKED DIAGNOSTICS
        v_state   = RETURNED_SQLSTATE,
        v_msg     = MESSAGE_TEXT,
        v_detail  = PG_EXCEPTION_DETAIL,
        v_hint    = PG_EXCEPTION_HINT,
        v_context = PG_EXCEPTION_CONTEXT;
    RAISE NOTICE E'Got exception:
        state  : %
        message: %
        detail : %
        hint   : %
        context: %', v_state, v_msg, v_detail, v_hint, v_context;
    RETURN NULL;
END;
$$
LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION main.delete_environment (
    _id          UUID      -- id          (1)
) RETURNS main.return_environment_type
AS $$
DECLARE
  res main.return_environment_type;
BEGIN
  DELETE FROM main.environments WHERE id = $1
  RETURNING id, signature, status, created_at, updated_at INTO res;
  RETURN res;
END;
$$
LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION main.update_environment_signature (
    _environment  UUID  -- (1)
) RETURNS main.return_environment_type
AS $$
DECLARE
  res main.return_environment_type;
  v_state   TEXT;
  v_msg     TEXT;
  v_detail  TEXT;
  v_hint    TEXT;
  v_context TEXT;
BEGIN
  UPDATE main.environments
  SET signature = (
    SELECT string_agg(i.signature, ','  ORDER BY i.signature)
    FROM main.environment_index_map AS ei
    INNER JOIN main.indexes AS i ON ei.index_id = i.id
    WHERE ei.environment = $1
    GROUP BY ei.environment
  )
  WHERE id = $1
  RETURNING id, signature, status, created_at, updated_at INTO res;
  RETURN res;
  EXCEPTION
  WHEN others
  THEN
    GET STACKED DIAGNOSTICS
        v_state   = RETURNED_SQLSTATE,
        v_msg     = MESSAGE_TEXT,
        v_detail  = PG_EXCEPTION_DETAIL,
        v_hint    = PG_EXCEPTION_HINT,
        v_context = PG_EXCEPTION_CONTEXT;
    RAISE NOTICE E'Got exception:
        state  : %
        message: %
        detail : %
        hint   : %
        context: %', v_state, v_msg, v_detail, v_hint, v_context;
    RETURN NULL;
END;
$$
LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION main.add_index_to_scenario (
    _index    UUID     -- (1)
  , _scenario UUID     -- (2)
) RETURNS main.return_environment_type
AS $$
DECLARE
  res main.return_environment_type;
  environment_id UUID;
  v_state   TEXT;
  v_msg     TEXT;
  v_detail  TEXT;
  v_hint    TEXT;
  v_context TEXT;
BEGIN
  -- We make sure the scenario and the index exist
  IF NOT EXISTS (SELECT FROM main.scenarios WHERE id = $2) THEN
    RAISE NOTICE 'Scenario does not exists';
    RETURN NULL;
  END IF;
  IF NOT EXISTS (SELECT FROM main.indexes WHERE id = $1) THEN
    RAISE NOTICE 'Index does not exists';
    RETURN NULL;
  END IF;

  -- If there is no environment for that scenario, create one
  IF NOT EXISTS (SELECT FROM main.scenario_environment_map WHERE scenario = $2) THEN
    INSERT INTO main.environments VALUES (default)
    RETURNING id INTO environment_id;
    -- and add it to the map
    INSERT INTO main.scenario_environment_map (scenario, environment) VALUES ($2, environment_id);
  ELSE
    SELECT environment FROM main.scenario_environment_map WHERE scenario = $2 INTO environment_id;
  END IF;

  -- And now assign that index to that environment
  INSERT INTO main.environment_index_map (environment, index_id) VALUES (environment_id, $1);

  SELECT * FROM main.update_environment_signature(environment_id) INTO res;
  -- Return the environment
  SELECT id, signature, status, created_at, updated_at FROM main.environments INTO res;
  RETURN res;
  EXCEPTION
  WHEN others
  THEN
    GET STACKED DIAGNOSTICS
        v_state   = RETURNED_SQLSTATE,
        v_msg     = MESSAGE_TEXT,
        v_detail  = PG_EXCEPTION_DETAIL,
        v_hint    = PG_EXCEPTION_HINT,
        v_context = PG_EXCEPTION_CONTEXT;
    RAISE NOTICE E'Got exception:
        state  : %
        message: %
        detail : %
        hint   : %
        context: %', v_state, v_msg, v_detail, v_hint, v_context;
    RETURN NULL;
END;
$$
LANGUAGE plpgsql;



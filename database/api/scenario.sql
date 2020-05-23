-- This type is used to return a scenario to the client. We skip some fields, like search
CREATE TYPE main.return_scenario_type AS (
    id UUID
  , name TEXT
  , tags TEXT[]
  , created_at TIMESTAMPTZ
  , updated_at TIMESTAMPTZ
);

CREATE OR REPLACE FUNCTION main.create_or_replace_scenario (
    _name    TEXT    -- name        (1)
  , _tags    TEXT[]  -- tags        (2)
  , _feature UUID    -- feature id  (3)
) RETURNS main.return_scenario_type
AS $$
DECLARE
  res main.return_scenario_type;
  v_state   TEXT;
  v_msg     TEXT;
  v_detail  TEXT;
  v_hint    TEXT;
  v_context TEXT;
BEGIN
  INSERT INTO main.scenarios (name, tags, feature) VALUES (
      $1 -- name
    , $2 -- tags
    , $3 -- feature
  )
  ON CONFLICT (feature, name) DO
    UPDATE
    SET   tags = EXCLUDED.tags
        , updated_at = NOW()
  RETURNING id, name, tags, created_at, updated_at INTO res;
  RETURN res;
  EXCEPTION
  WHEN others THEN
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

CREATE OR REPLACE FUNCTION main.create_scenario (
    _name    TEXT    -- name        (1)
  , _tags    TEXT[]  -- tags        (2)
  , _feature UUID    -- feature id  (3)
) RETURNS main.return_scenario_type
AS $$
DECLARE
  res main.return_scenario_type;
  v_state   TEXT;
  v_msg     TEXT;
  v_detail  TEXT;
  v_hint    TEXT;
  v_context TEXT;
BEGIN
  INSERT INTO main.scenarios (name, tags, feature) VALUES (
      $1 -- name
    , $2 -- tags
    , $3 -- feature
  )
  RETURNING id, name, tags, created_at, updated_at INTO res;
  RETURN res;
  -- EXCEPTION
  -- WHEN others THEN
  --     GET STACKED DIAGNOSTICS
  --         v_state   = RETURNED_SQLSTATE,
  --         v_msg     = MESSAGE_TEXT,
  --         v_detail  = PG_EXCEPTION_DETAIL,
  --         v_hint    = PG_EXCEPTION_HINT,
  --         v_context = PG_EXCEPTION_CONTEXT;
  --     RAISE NOTICE E'Got exception:
  --         state  : %
  --         message: %
  --         detail : %
  --         hint   : %
  --         context: %', v_state, v_msg, v_detail, v_hint, v_context;
  --     RETURN NULL;
END;
$$
LANGUAGE plpgsql;

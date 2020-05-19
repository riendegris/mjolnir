-- This type is used to return a step to the client. We skip some fields, like search
CREATE TYPE main.return_step_type AS (
    id UUID
  , step_type main.step_type
  , value TEXT
  , docstring TEXT
  , created_at TIMESTAMPTZ
  , updated_at TIMESTAMPTZ
);

CREATE OR REPLACE FUNCTION main.create_or_replace_step (
    _id           UUID             -- id          (1)
  , _type         main.step_type   -- step_type   (2)
  , _value        TEXT             -- value       (3)
  , _docstring    TEXT             -- docstring   (4)
) RETURNS main.return_step_type
AS $$
DECLARE
  res main.return_step_type;
  v_state   TEXT;
  v_msg     TEXT;
  v_detail  TEXT;
  v_hint    TEXT;
  v_context TEXT;
BEGIN
  INSERT INTO main.steps VALUES (
      $1 -- id
    , $2 -- step_type
    , $3 -- value
    , $4 -- docstring
  )
  ON CONFLICT (id) DO
    UPDATE
    SET   step_type  = EXCLUDED.step_type
        , value      = EXCLUDED.value
        , docstring  = EXCLUDED.docstring
        , updated_at = NOW();
  SELECT id, step_type, value, docstring, created_at, updated_at FROM main.steps WHERE id = $1 INTO res;
  RETURN res;
  EXCEPTION WHEN others THEN
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
END;
$$
LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION main.add_step_to_scenario (
    INOUT _scenario_id UUID   -- sceranio id (1)
  , _step_id UUID             -- step id     (2)
  , OUT _updated_at TIMESTAMPTZ)
AS $$
DECLARE
    v_state   TEXT;
    v_msg     TEXT;
    v_detail  TEXT;
    v_hint    TEXT;
    v_context TEXT;
BEGIN
  -- Relying on foreign key constraints to ensure scenario id and step id exists
  INSERT INTO main.scenario_step_map
  VALUES (
      $1 -- scerario id
    , $2 -- step id
  );
  UPDATE main.scenarios
  SET updated_at = NOW()
  WHERE id = $1
  RETURNING updated_at INTO _updated_at;
  EXCEPTION WHEN others THEN
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
END;
$$
LANGUAGE plpgsql;

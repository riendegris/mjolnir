-- This type is used to return a feature to the client. We skip some fields, like search
CREATE TYPE main.return_feature_type AS (
    id UUID
  , name TEXT
  , description TEXT
  , tags TEXT[]
  , created_at TIMESTAMPTZ
  , updated_at TIMESTAMPTZ
);

-- This function returns a bit of information about each feature in the database,
-- using a string to filter features based on full text search.
CREATE OR REPLACE FUNCTION main.feature_search(
  TEXT -- query
)
RETURNS TABLE (
    _id UUID
  , _name VARCHAR(256)
  , _description TEXT
  , _tags TEXT[]
  , _created_at TIMESTAMPTZ
  , _updated_at TIMESTAMPTZ)
AS $$
BEGIN
  RETURN QUERY
  SELECT f.id, f.name, f.description, f.tags, f.created_at, f.updated_at
  FROM main.features AS f, websearch_to_tsquery($1) AS query
  WHERE query @@ f.search
  ORDER BY ts_rank(f.search, query) DESC;
END;
$$
LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION main.create_or_replace_feature (
    _name        TEXT      -- name        (1)
  , _description TEXT      -- description (2)
  , _tags        TEXT[]    -- tags        (3)
) RETURNS main.return_feature_type
AS $$
DECLARE
  res main.return_feature_type;
  v_state   TEXT;
  v_msg     TEXT;
  v_detail  TEXT;
  v_hint    TEXT;
  v_context TEXT;
BEGIN
  INSERT INTO main.features (name, description, tags) VALUES (
      $1  -- name
    , $2  -- description
    , $3  -- tags
  )
  ON CONFLICT (name) DO
    UPDATE
    SET   description = EXCLUDED.description
        , tags = EXCLUDED.tags
        , updated_at = NOW()
  RETURNING id, name, description, tags, created_at, updated_at INTO res;
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

CREATE OR REPLACE FUNCTION main.delete_feature (
    _id          UUID      -- id          (1)
) RETURNS main.return_feature_type
AS $$
DECLARE
  res main.return_feature_type;
BEGIN
  DELETE FROM main.features WHERE id = $1
  RETURNING id, name, description, tags, created_at, updated_at INTO res;
  RETURN res;
END;
$$
LANGUAGE plpgsql;

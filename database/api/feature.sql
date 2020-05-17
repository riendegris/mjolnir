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
    INOUT _id UUID      -- id          (1)
  , TEXT                -- name        (2)
  , TEXT                -- description (3)
  , TEXT                -- tags        (4)
  , OUT _updated_at TIMESTAMPTZ)
AS $$
BEGIN
  INSERT INTO main.features VALUES (
      $1                        -- id
    , $2                        -- name
    , $3                        -- description
    , string_to_array($4, ',')  -- tags
  )
  ON CONFLICT (id) DO
    UPDATE
    SET   name = EXCLUDED.name
        , description = EXCLUDED.description
        , tags = EXCLUDED.tags
        , updated_at = NOW()
    RETURNING updated_at INTO _updated_at;
END;
$$
LANGUAGE plpgsql;

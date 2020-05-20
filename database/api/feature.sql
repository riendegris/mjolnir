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
    _id          UUID      -- id          (1)
  , _name        TEXT      -- name        (2)
  , _description TEXT      -- description (3)
  , _tags        TEXT[]    -- tags        (4)
) RETURNS main.return_feature_type
AS $$
DECLARE
  res main.return_feature_type;
BEGIN
  INSERT INTO main.features VALUES (
      $1  -- id
    , $2  -- name
    , $3  -- description
    , $4  -- tags
  )
  ON CONFLICT (id) DO
    UPDATE
    SET   name = EXCLUDED.name
        , description = EXCLUDED.description
        , tags = EXCLUDED.tags
        , updated_at = NOW();
  SELECT id, name, description, tags, created_at, updated_at FROM main.features WHERE id = $1 INTO res;
  RETURN res;
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

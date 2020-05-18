-- This type is used to return a scenario to the client. We skip some fields, like search
CREATE TYPE main.return_scenario_type AS (
    id UUID
  , name TEXT
  , tags TEXT[]
  , created_at TIMESTAMPTZ
  , updated_at TIMESTAMPTZ
);

CREATE OR REPLACE FUNCTION main.create_or_replace_scenario (
    _id UUID      -- scenario id (1)
  , _name TEXT    -- name        (2)
  , _tags TEXT[]  -- tags        (3)
  , _feature UUID -- feature id  (4)
) RETURNS main.return_scenario_type
AS $$
DECLARE
  res main.return_scenario_type;
BEGIN
  INSERT INTO main.scenarios VALUES (
      $1 -- id
    , $4 -- feature
    , $2 -- name
    , $3 -- tags
  )
  ON CONFLICT (id) DO
    UPDATE
    SET   name = EXCLUDED.name
        , feature = EXCLUDED.feature
        , tags = EXCLUDED.tags
        , updated_at = NOW();
  SELECT id, name, tags, created_at, updated_at FROM main.scenarios WHERE id = $1 INTO res;
  RETURN res;
END;
$$
LANGUAGE plpgsql;

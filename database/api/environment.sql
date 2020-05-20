-- This type is used to return a environment to the client
CREATE TYPE main.return_environment_type AS (
    id UUID
  , signature TEXT
  , status main.index_status
  , created_at TIMESTAMPTZ
  , updated_at TIMESTAMPTZ
);

-- This type is used to return an index to the client
CREATE TYPE main.return_index_type AS (
    id UUID
  , signature TEXT
  , index_type TEXT
  , data_source TEXT
  , region      TEXT
  , filepath    TEXT
  , status      main.index_status
  , created_at TIMESTAMPTZ
  , updated_at TIMESTAMPTZ
);

CREATE OR REPLACE FUNCTION main.create_or_replace_environment (
    _id          UUID                 -- id          (1)
  , _signature   TEXT                 -- signature   (2)
  , _status      main.index_status    -- status      (3)
) RETURNS main.return_environment_type
AS $$
DECLARE
  res main.return_environment_type;
BEGIN
  INSERT INTO main.environments VALUES (
      $1  -- id
    , $2  -- signature
    , $3  -- status
  )
  ON CONFLICT (id) DO
    UPDATE
    SET   signature = EXCLUDED.signature
        , status = EXCLUDED.status
        , tags = EXCLUDED.tags
        , updated_at = NOW();
  SELECT id, signature, status, created_at, updated_at FROM main.environments WHERE id = $1 INTO res;
  RETURN res;
END;
$$
LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION main.create_or_replace_index (
    _id          UUID    -- id          (1)
  , _signature   TEXT    -- signature   (2)
  , _index_type  TEXT    -- index type  (3)
  , _data_source TEXT    -- data source (4)
  , _region      TEXT    -- region      (5)
) RETURNS main.return_index_type
AS $$
DECLARE
  res main.return_index_type;
BEGIN
  INSERT INTO main.indexes VALUES (
      $1  -- id
    , $2  -- signature
    , $3  -- index type
    , $4  -- data source
    , $5  -- region
  )
  ON CONFLICT (id) DO
    UPDATE
    SET   signature = EXCLUDED.signature
        , index_type = EXCLUDED.index_type
        , data_source = EXCLUDED.data_source
        , region = EXCLUDED.region
        , updated_at = NOW();
  SELECT id, signature, index_type, data_source, region, filepath, status, created_at, updated_at
  FROM main.indexes WHERE id = $1 INTO res;
  RETURN res;
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

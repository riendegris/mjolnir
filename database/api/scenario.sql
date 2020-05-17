CREATE OR REPLACE FUNCTION main.create_or_replace_scenario (
    INOUT _id UUID      -- scenario id (1)
  , TEXT                -- name        (2)
  , TEXT                -- description (3)
  , TEXT                -- tags        (4)
  , UUID                -- feature id  (5)
  , OUT _updated_at TIMESTAMPTZ)
AS $$
BEGIN
  INSERT INTO main.scenarios VALUES (
      $1                        -- id
    , $5                        -- feature
    , $2                        -- name
    , $3                        -- description
    , string_to_array($4, ',')  -- tags
  )
  ON CONFLICT (id) DO
    UPDATE
    SET   name = EXCLUDED.name
        , feature = EXCLUDED.feature
        , description = EXCLUDED.description
        , tags = EXCLUDED.tags
        , updated_at = NOW()
    RETURNING updated_at INTO _updated_at;
END;
$$
LANGUAGE plpgsql;

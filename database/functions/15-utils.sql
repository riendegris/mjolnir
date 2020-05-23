-- For aggregating tags
-- See https://stackoverflow.com/questions/31210790/indexing-an-array-for-full-text-search
--
CREATE OR REPLACE FUNCTION array2string(TEXT[])
  RETURNS TEXT LANGUAGE SQL IMMUTABLE AS $$SELECT array_to_string($1, ',')$$;

CREATE OR REPLACE FUNCTION random_signature()
  RETURNS TEXT LANGUAGE SQL IMMUTABLE AS $$SELECT MD5(RANDOM()::TEXT)$$;

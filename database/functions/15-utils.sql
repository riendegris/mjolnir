-- For aggregating tags
-- See https://stackoverflow.com/questions/31210790/indexing-an-array-for-full-text-search
--
CREATE OR REPLACE FUNCTION textarr2text(TEXT[])
  RETURNS TEXT LANGUAGE SQL IMMUTABLE AS $$SELECT array_to_string($1, ',')$$;

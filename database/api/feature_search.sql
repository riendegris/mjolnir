-- This function returns a bit of information about each document,
-- using a string to filter documents based on full text search.
CREATE OR REPLACE FUNCTION main.feature_search(
  TEXT -- query
)
RETURNS TABLE (_id UUID,
  _title VARCHAR(256),
  _description TEXT,
  _tags TEXT[],
  _created_at TIMESTAMPTZ,
  _updated_at TIMESTAMPTZ)
AS $$
BEGIN
  RETURN QUERY
  SELECT f.id, f.title, f.description, f.tags, f.created_at, f.updated_at
  FROM main.features AS f, websearch_to_tsquery($1) AS query
  WHERE query @@ f.search
  ORDER BY ts_rank(f.search, query) DESC;
END;
$$
LANGUAGE plpgsql;

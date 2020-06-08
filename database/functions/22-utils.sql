CREATE OR REPLACE FUNCTION main.tg_notify ()
  RETURNS TRIGGER
  LANGUAGE plpgsqL
AS $$
DECLARE
  channel TEXT := TG_ARGV[0];
BEGIN
  PERFORM pg_notify(channel, row_to_json(NEW)::text);
  RETURN NULL;
END;
$$;

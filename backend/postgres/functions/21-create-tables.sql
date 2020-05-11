SET CLIENT_MIN_MESSAGES TO INFO;
SET CLIENT_ENCODING = 'UTF8';
DROP SCHEMA IF EXISTS main CASCADE;
CREATE SCHEMA main AUTHORIZATION odin;
GRANT ALL ON SCHEMA main TO odin;
SET SEARCH_PATH = main;

CREATE TYPE main.file_status AS ENUM ('not_available', 'download_in_progress', 'available', 'download_error');

CREATE TABLE main.env_bano (
  id VARCHAR(256) PRIMARY KEY,
  description VARCHAR(256) NOT NULL
);

ALTER TABLE main.env_bano OWNER TO odin;

CREATE TABLE main.env_bano_item (
  id VARCHAR(256) PRIMARY KEY,
  filename VARCHAR(256) DEFAULT '',
  md5 VARCHAR(256) DEFAULT '',
  filesize DOUBLE PRECISION NOT NULL DEFAULT 0.0, -- Expressed in kilobytes
  filestatus main.file_status DEFAULT 'not_available',
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE main.env_bano_item OWNER TO odin;

-- This table maps bano to bano_items
CREATE TABLE main.env_bano_map (
  env VARCHAR(256) REFERENCES main.env_bano(id),
  item VARCHAR(256) REFERENCES main.env_bano_item(id)
);

ALTER TABLE main.env_bano_map OWNER TO odin;

-- help from https://tapoueh.org/blog/2018/07/postgresql-listen/notify/
-- Trigger notification for messaging to PG Notify
CREATE FUNCTION notify_trigger() RETURNS trigger AS $trigger$
DECLARE
  rec RECORD;
  payload TEXT;
  column_name TEXT;
  column_value TEXT;
  payload_items TEXT[];
BEGIN
  -- Set record row depending on operation
  CASE TG_OP
  WHEN 'INSERT', 'UPDATE' THEN
     rec := NEW;
  WHEN 'DELETE' THEN
     rec := OLD;
  ELSE
     RAISE EXCEPTION 'Unknown TG_OP: "%". Should not occur!', TG_OP;
  END CASE;

  -- Get required fields
  FOREACH column_name IN ARRAY TG_ARGV LOOP
    EXECUTE format('SELECT $1.%I::TEXT', column_name)
    INTO column_value
    USING rec;
    payload_items := array_append(payload_items, '"' || replace(column_name, '"', '\"') || '":"' || replace(column_value, '"', '\"') || '"');
  END LOOP;

  -- Build the payload
  payload := ''
              || '{'
              || '"timestamp":"' || CURRENT_TIMESTAMP                    || '",'
              || '"operation":"' || TG_OP                                || '",'
              || '"schema":"'    || TG_TABLE_SCHEMA                      || '",'
              || '"table":"'     || TG_TABLE_NAME                        || '",'
              || '"data":{'      || array_to_string(payload_items, ',')  || '}'
              || '}';

  -- Notify the channel
  PERFORM pg_notify('banos', payload);

  RETURN rec;
END;
$trigger$ LANGUAGE plpgsql;

CREATE TRIGGER banos_notify
AFTER INSERT OR UPDATE ON main.env_bano_item
FOR EACH ROW
EXECUTE PROCEDURE notify_trigger('id', 'filestatus');

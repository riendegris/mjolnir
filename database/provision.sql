INSERT INTO main.index_types VALUES ('admins');
INSERT INTO main.index_types VALUES ('streets');
INSERT INTO main.index_types VALUES ('addresses');
INSERT INTO main.index_types VALUES ('stops');
INSERT INTO main.index_types VALUES ('public_pois');
INSERT INTO main.index_types VALUES ('private_pois');

INSERT INTO main.data_sources VALUES ('osm');
INSERT INTO main.data_sources VALUES ('cosmogony');
INSERT INTO main.data_sources VALUES ('bano');

INSERT INTO main.index_type_data_source VALUES ('admins', 'cosmogony');
INSERT INTO main.index_type_data_source VALUES ('admins', 'osm');
INSERT INTO main.index_type_data_source VALUES ('streets', 'osm');
INSERT INTO main.index_type_data_source VALUES ('addresses', 'bano');

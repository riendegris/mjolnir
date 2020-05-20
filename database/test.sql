SELECT * FROM main.create_or_replace_feature ('1b195a43-b929-4d65-8a44-420f83475bae', 'search for pokemons', 'searching for pokemons is a fun activity', '{"pokemon", "search"}');
SELECT * FROM main.create_or_replace_feature ('2b195a43-b929-4d65-8a44-420f83475bae', 'search for barbarians', 'searching for barbarians is a fun activity', '{"pokemon", "search"}');
SELECT * FROM main.create_or_replace_feature ('3b195a43-b929-4d65-8a44-420f83475bae', 'search for pirates', 'searching for pirates is a fun activity', '{"pokemon", "search"}');
SELECT * FROM main.create_or_replace_scenario ('7a92a064-add8-4be3-a764-960798fea22d', 'in the park', '{"park"}', '3b195a43-b929-4d65-8a44-420f83475bae');
SELECT * FROM main.create_or_replace_step ('4994a2ce-74a2-4888-afd5-36b44ada553b', 'given', 'I am walking in the park', '');
SELECT * FROM main.add_step_to_scenario('7a92a064-add8-4be3-a764-960798fea22d', '4994a2ce-74a2-4888-afd5-36b44ada553b');
SELECT * FROM main.delete_feature ('1b195a43-b929-4d65-8a44-420f83475bae');
-- SELECT * FROM main.create_or_replace_step ('84858bca-5772-4649-b887-fafbdb8949c0', 'when', 'I breath in', '');
-- SELECT * FROM main.add_step_to_scenario('7a92a064-add8-4be3-a764-960798fea22d', '84858bca-5772-4649-b887-fafbdb8949c0');
-- SELECT * FROM main.create_or_replace_step ('4550709c-4b53-4b98-a67b-e21d5810d136', 'then', 'I am healthier', '');
-- SELECT * FROM main.add_step_to_scenario('7a92a064-add8-4be3-a764-960798fea22d', '4550709c-4b53-4b98-a67b-e21d5810d136');


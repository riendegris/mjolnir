#!/bin/sh

endpoint="http://localhost:3030/graphql"

curl_cmd="curl -X POST -H 'Content-Type: application/json'"
curl_cmd="${curl_cmd} --data '{ \"query\": \"mutation($bid: String!, $iid: String!) { addBanoItem(banoId: $bid, itemId: $iid) { ok, error, items { id }} }\", \"variables\": {\"bid\": \"idf\", \"iid\": \"78\" } }'"
curl_cmd="${curl_cmd} ${endpoint}"

#
# # This is an alternative that uses a shape to constrain the results to a geographic area.
# # curl_cmd="curl -s -d @idf.geojson -X POST \"http://localhost:4000/autocomplete?q=stade&pt_dataset[]=stif&_debug=true\" --header \"Content-Type:application/json\""
#
echo "${curl_cmd}"
resp=$(eval ${curl_cmd})
echo "${resp}" | jq '.'
# echo "${resp}" | jq '[ .features[] | { "label": .properties.geocoding.label, "type": .properties.geocoding.type, "zone_type": .properties.geocoding.zone_type, "level": .properties.geocoding.level } ]'
# echo "${resp}" | jq '[ .features[] | { "label": .properties.geocoding.label, "type": .properties.geocoding.type, "zone_type": .properties.geocoding.zone_type, "level": .properties.geocoding.level, "admins": .properties.geocoding.administrative_regions | length } ]'

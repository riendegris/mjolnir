@regression @sanity
Feature: Some minimal acceptance tests in France

  These scenarios represent a baseline of acceptance tests for bragi

  Background:
    Given I am indexing admins with cosmogony from france
    And I am indexing streets with osm from ile-de-france
    And I am indexing addresses with bano from ile-de-france
    And I am indexing public_pois with osm from ile-de-france

  @admin @city
  Scenario: Searching for an administrative area
    When I search for 'paris'
    Then I find 'Paris' of type 'city' within the first 2 results

  @street @withAdmin @properNoun @fullSpell
  Scenario: Searching for a street using the full street name and the city
    When I search for 'rue hector malot paris'
    Then I find 'Rue Hector Malot (Paris)' of type 'street' within the first 2 results

  @street @withAdmin @properNoun @fullSpell
  Scenario: Searching for an address using the full street name and the city
    When I search for '20 rue hector malot paris'
    Then I find '20 Rue Hector Malot (Paris)' of type 'address' within the first 2 results

@regression @sanity
Feature: Some minimal acceptance tests in France

  These scenarios represent a baseline of acceptance tests for bragi

  Background:
    Given I am indexing admins with cosmogony from france
    And I am indexing streets with osm from ile-de-france
    And I am indexing addresses with bano from ile-de-france
    And I am indexing public_pois with osm from ile-de-france

  @regression @sanity
  Scenario: Searching for various
    When I search for <query>
    Then I find <label> of type <type> within the first <count> results
    Examples:
      | query                     | label                       | type      | count |
      | paris                     | Paris                       | city      | 2     |
      | rue hector malot paris    | Rue Hector Malot (Paris)    | street    | 2     |
      | 20 rue hector malot paris | 20 Rue Hector Malot (Paris) | address   | 2     |
      | notre dame                | Notre-Dame (Paris)          | poi       | 2     |
      | chatelet paris            | Châtelet (Paris)            | stop_area | 2     |

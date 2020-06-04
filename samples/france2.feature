Feature: More tests

  These scenarios represent a baseline of acceptance tests for bragi

  Scenario: Searching for a needle in a haystack
    Given I am indexing admins with cosmogony from france
    And I am indexing streets with osm from ile-de-france
    And I am indexing addresses with bano from ile-de-france
    And I am indexing public_pois with osm from ile-de-france
    When I search for 'needle'
    Then I find the haystack is prickly

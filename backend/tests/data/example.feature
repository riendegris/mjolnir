@awesome
Feature: Searching for features

  We are evaluating different scenarios for loading and then fetching features

  Scenario: Loading a feature and finding it
    Given I am loading a feature from file './tests/data/example.feature'
    When I search for the feature by id
    Then I find that feature and verify its name

  Scenario: Loading a feature and finding scenarios
    Given I am loading a feature from file './tests/data/example.feature'
    When I search for the scenarios by id
    Then I find that I have the correct number of scenarios

  Scenario: Loading a feature and finding steps
    Given I am loading a feature from file './tests/data/example.feature'
    When I search for the steps belonging to the first scenario
    Then I find that I have the correct number of steps

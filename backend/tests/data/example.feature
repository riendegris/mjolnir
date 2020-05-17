@awesome
Feature: Searching for features
  We are evaluating different scenarios for loading and then fetching features
  Scenario: Loading and Finding
    Given I am loading a feature from file './tests/data/example.feature'
    When I search for the feature by id
    Then I can find that feature and verify its name

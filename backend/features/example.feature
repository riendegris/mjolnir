Feature: Searching for features

  Scenario: Loading and Finding
    Given I am loading a feature from file "./tests/data/simple.feature"
    When I search for the feature by id
    Then I can find that feature and verify its name

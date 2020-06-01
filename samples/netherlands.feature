@minimal
Feature: Some minimal acceptance tests in France

  These scenarios represent a baseline of acceptance tests for bragi

  Background:
    Given I am indexing admins with cosmogony from france
    And I am indexing streets with osm from ile-de-france
    And I am indexing addresses with bano from ile-de-france
    And I am indexing public_pois with osm from ile-de-france

  Scenario
    When I search for <query>
    Then I find <label> of type <type>
    Examples:
      | query                     | label                       | type      |
      | paris                     | Paris                       | city      |
      | rue hector malot paris    | Rue Hector Malot (Paris)    | street    |
      | 20 rue hector malot paris | 20 Rue Hector Malot (Paris) | address   |
      | notre dame                | Notre-Dame (Paris)          | poi       |
      | chatelet paris            | Châtelet (Paris)            | stop_area |

nl-c;bragi;;;amsterdam;Amsterdam;city;10;
nl-c;bragi;;;zaandam;Zaandam;city;10;
nl-c;bragi;;;zaandijk;Zaandijk;city;10;
nl-c;bragi;;;venlo;Venlo;city;10;
nl-c;bragi;2;;venlo venlo;Venlo (Venlo);stop_area;;
nl-c;bragi;2;;centraal station amsterdam;Amsterdam, Centraal Station (Amsterdam);stop_area;;
nl-c;bragi;;;elvis presleyslaan;Elvis Presleystraat (Zaandijk);street;;
nl-c;bragi;;;Elvis Presleylaan;Elvis Presleylaan (Goes);street;;
nl-c;bragi;;;elvis presleystraat;Elvis Presleystraat (Almere);street;;"Middelburg=48k, Almere=196k, Arnhem=152k, Zaandijk=8k"
nl-c;bragi;2;;elvis presleystraat;Elvis Presleystraat (Arnhem);street;;"Middelburg=48k, Almere=196k, Arnhem=152k, Zaandijk=8k"
nl-c;bragi;3;;elvis presleystraat;Elvis Presleystraat (Middelburg);street;;"Middelburg=48k, Almere=196k, Arnhem=152k, Zaandijk=8k"
nl-c;bragi;4;;elvis presleystraat;Elvis Presleystraat (Zaandijk);street;;"Middelburg=48k, Almere=196k, Arnhem=152k, Zaandijk=8k"
nl-c;bragi;;;schoolstraat utrecht;Schoolstraat (Utrecht);street;;
nl-c;bragi;;;schoolstraat 4 utrecht;Schoolstraat 4 (Utrecht);housenumber;;"existe sur internal"
nl-c;bragi;;;koningslaan utrecht;Koningslaan (Utrecht);street;;
nl-c;bragi;;;koningslaan 4 utrecht;Koningslaan 4 (Utrecht);housenumber;;"existe sur internal"
nl-c;bragi;;;De Ruijterkade 105-1 amsterdam;De Ruijterkade 105-1 (Amsterdam);housenumber;;"existe sur Open Addresses avec la m^m coord que le 105-2"
nl-c;bragi;;;De Ruijterkade 105-2 amsterdam;De Ruijterkade 105-2 (Amsterdam);housenumber;;"existe sur Open Addresses avec la m^m coord que le 105-1"
nl-c;bragi;;;De Ruijterkade 150 amsterdam;De Ruijterkade 150 (Amsterdam);housenumber;;"existe sur DEV/Prod"

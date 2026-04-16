use crate::model;
 
 
 pub fn get_locations() -> Vec<model::Location> {
    vec![
        model::Location {
            name: "Paris".to_string(),
            lat: 48.85,
            lon: 2.35,
        },             
        model::Location {
            name: "Bruyeres".to_string(),
            lat: 49.1576,
            lon: 2.3258,
        },
          model::Location {
            name: "Saint Ju".to_string(),
            lat: 44.8841,
            lon: 1.3232,
        },
           model::Location {
            name: "Biskra".to_string(),
            lat: 34.8333,
            lon: 5.7333,
        },
           model::Location {
            name: "Mimizan".to_string(),
            lat: 44.2013,
            lon: -1.2287,
        },
           model::Location {
            name: "La Rochelle".to_string(),
            lat: 46.1667,
            lon: -1.15,
        },
           model::Location {
            name: "Toulouse".to_string(),
            lat: 43.6045,
            lon: 1.4440,
        },
        model::Location {
            name: "Brest".to_string(),
            lat: 48.3903,
            lon: -4.4863,
        },
        model::Location {
            name: "Marseille".to_string(),
            lat: 43.2965,
            lon: 5.3698,
        },
        model::Location {
            name: "Lille".to_string(),
            lat: 50.6292,
            lon: 3.0573,
        },
        model::Location {
            name: "Jersey".to_string(),
            lat: 49.19,
            lon: -2.10,
        },

    ]
}
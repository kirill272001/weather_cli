use weather_cli::weather::city_to_coords; 
use weather_cli::time_service::city_to_timezone;

#[test]
fn test_known_city_to_timezone() {
    assert_eq!(city_to_timezone("Kyiv"), Some("Europe/Kiev"));
    assert_eq!(city_to_timezone("London"), Some("Europe/London"));
}

#[test]
fn test_unknown_city_timezone() {
    assert_eq!(city_to_timezone("MarsCity"), None);
}
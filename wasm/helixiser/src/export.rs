use crate::helix::Helix;

/// Generate a URL to helix model in the Helxiser web application
///
/// # Examples
/// ```
///    use open::that;
///    use helixiser::helix::{ Helix, Handedness };
///    use helixiser::export::helixiser_web_link;
///
///    let helix_1 = Helix {
///        radius: 0.5,
///        rise: 0.05,
///        frequency: 10.,
///        unit_size: 0.18,
///        offset: 0.,
///        rotation: 0.,
///        handedness: Handedness::Right,
///    };
///
///    let link = helixiser_web_link("Rust", vec![helix_1]);
///    println!("Helixiser Web-link: {}", link);
///    // uncomment line below to open the model in browser
///    // open::that(link);
/// ```
pub fn helixiser_web_link(name: &str, helix_family: Vec<Helix>) -> String {
    let mut web_link: String = String::new();
    // Add the base URL
    web_link.push_str("https://nemoandrea.github.io/helixiser/#");
    // Add the meta fields (name, n, m, scale)
    web_link.push_str(&format!("name={}###", name));
    // specify the helices
    for helix in helix_family {
        web_link.push_str(&format!("&radius={}", helix.radius));
        web_link.push_str(&format!("&rise={}", helix.rise));
        web_link.push_str(&format!("&frequency={}", helix.frequency));
        web_link.push_str(&format!("&unit_size={}", helix.unit_size));
        web_link.push_str(&format!("&rotation={}", helix.rotation));
        web_link.push_str(&format!("&offset={}", helix.offset));
    }
    return web_link
}
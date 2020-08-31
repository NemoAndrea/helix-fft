use wasm_bindgen::prelude::*;
use crate::analytic_diffraction::*;
use crate::display::adjust_contrast;

// Functions that use the WASM bindings to be called from javascript
// For some functions this will result in a simple wrapper, but it keeps things organised


#[wasm_bindgen]
#[derive(Serialize, Deserialize)]  // just needed to hold the JS values
pub struct Element {
    radius: String,
    rise: String,
    frequency: String,
    unit_size: String,
    offset: String,
    rotation: String,
    handedness: String
}

#[wasm_bindgen]  // function that converts JsValue to Helix Struct and then calls diff_analytic
pub fn wasm_diffraction_analytic(helix_family: &JsValue, n_range: u8, m_range: u8, scale: f64, raster_size: u32) -> Vec<f64> {
    // helix_family is an array of dicts in JS.
    let JS_helices: Vec<Element> = helix_family.into_serde().unwrap();
    let mut helices: Vec<Helix> = Vec::new();

    // We now convert our JS helices into our Helix struct
    for JS_helix in &JS_helices {
        // convert handedness string to enum 'Handedness'
        let mut hand: Handedness = Handedness::Right;
        let JS_string = JS_helix.handedness.parse::<String>().unwrap_or("right".clone().parse().unwrap());
        if let JS_string = String::from("left") {
            hand = Handedness::Left
        } else { hand = Handedness::Right };

        helices.push(Helix {
            radius: JS_helix.radius.parse::<f64>().unwrap_or(0 as f64),
            rise: JS_helix.rise.parse::<f64>().unwrap_or(0 as f64),
            frequency: JS_helix.frequency.parse::<f64>().unwrap_or(0 as f64),
            unit_size: JS_helix.unit_size.parse::<f64>().unwrap_or(0 as f64),
            offset: JS_helix.offset.parse::<f64>().unwrap_or(0 as f64),
            rotation: JS_helix.rotation.parse::<f64>().unwrap_or(0 as f64),
            handedness: hand,
        });
    }

    // now we call the function that actually draws the image and returns it
    diff_analytic(helices, n_range, m_range, scale, raster_size)
}


#[wasm_bindgen]  // just a wrapper for the main adjust_contrast() function
pub fn wasm_adjust_contrast(image_data: Vec<f64>, offset: f64, min: f64, max: f64) -> Vec<f64> {
    adjust_contrast(image_data, offset, min, max)
}
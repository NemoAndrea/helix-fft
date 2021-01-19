// pub fn key_red(mut frame: Vec<u8>) -> Vec<u8>{
//     let tolerance: u8 = 100;
//     let mut r: u8;
//     let mut g: u8;
//     let mut b: u8;
//     for i in (0..frame.len()).step_by(4) {
//         r = frame[i];
//         g = frame[i + 1];
//         b = frame[i + 2];
//         if (r >= 249 - tolerance && r <= 249 + tolerance) && (g >= 6 - tolerance && g <= 6 + tolerance) && (b >= 0 - tolerance && b <= 0 + tolerance) {
//             frame[i + 3] = 0;
//         }
//     }
//     return frame
// }
# Fast Fourier Transform of Helixiser

Helixiser can compute the FFT of a realspace image of a helical structure to produce a diffraction pattern of said structure. The code for this section (to read along) can be found in [fft_2D.rs](../wasm/src/fft_2D.rs). The function that handles the passing of FFT between JS and WebAssembly can be found in [wasm_functions.rs](../wasm/src/wasm_functions.rs) and called `wasm_FFT()`.

### Details

To compute the FFT we do the following:

1. Pad the image with the mean value to powers of 2. The largest resulting dimension will  be used as `width` and `height` for the FFT. The final padded image is therefore square.
2. Take the 2D Fourier transform of padded image. The 2D FFT is built up from the 1D FFT that comes with the `rustfft` crate. We use the `radix4` algorithm of `rustfft` to do the computation.
3. We take the (complex-valued) FFT of the padded image, take the **norm**, then take the **log values** and then return this to be plotted.

> Currently, because the Rust->WebAssembly doesn't properly support complex multi-valued return, we use a rather fragile construction where we append the width and height of the FFT image to the rgba values of the image when returning it to JS. Ideally we would return these dimensions as separate variables. It has no consequences for the functionality, but it is important to notice if you are going through the code or wish to modify it.
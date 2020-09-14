export function upload_to_rgba( dataURL ){
    let image = new Image();   //dummy image
    image.src = dataURL;

    // if (image.width !== image.height) {
    //     throw "Image aspect ratio is not square"
    // }
    // if (image.width > 3000 || image.height > 3000) {
    //     throw "Image dimensions are too large"
    // }

    const canvas = document.createElement('canvas');  // add a canvas (but dont show it)
    canvas.width =image.width;
    canvas.height =image.height;
    const localctx = canvas.getContext("2d");

    localctx.drawImage(image, 0,0);

    const rgba = localctx.getImageData(0,0,image.width,image.height);
    return [ rgba, image.width, image.height ]
}

// convert an image object to a dataURL using a canvas that isnt part of DOM.
/**
 * @return {string}
 */
export function ImageData_to_dataURL( Image ) {
    const canvas = document.createElement('canvas');  // add a canvas (but dont show it)
    canvas.width = Image.width;
    canvas.height = Image.height;
    const localctx = canvas.getContext("2d");
    localctx.putImageData( Image, 0, 0 );
    return canvas.toDataURL()
}


// make enum  // current value (string) is not used currently
export const ImageType = {
    ANALYTIC: 'var(--primary)',
    EXPERIMENTAL: 'var(--secondary)',
};
Object.freeze(ImageType);

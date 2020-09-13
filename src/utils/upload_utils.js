export function upload_to_rgba( dataURL ){
    let image = new Image();   //dummy image
    image.src = dataURL;

    if (image.width !== image.height) {
        throw "Image aspect ratio is not square"
    }
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

// make enum
export const ImageType = {
    ANALYTIC: 1,
    EXPERIMENTAL: 2,
};
Object.freeze(ImageType);

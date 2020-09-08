export function upload_to_rgba( dataURL ){
    const canvas = document.createElement('canvas');  // add a canvas (but dont show it)
    canvas.width = 512;
    canvas.height = 512;

    const localctx = canvas.getContext("2d");

    let image = new Image();   //dummy image
    image.src = dataURL;

    localctx.drawImage(image, 0,0);
    return localctx.getImageData(0,0,512,512).data
}

import { identity, multiply, subset, index, max, min, subtract, zeros, range, exp, complex, add, squeeze, square, abs} from '../../ext/math.min.js'
import { besselj } from '../../ext/bessel'

export function fft_analytic(radius, rise, frequency, unit_size, rastersize, n_range=10, m_range=1, scale=0.003) {
    let raw_image = zeros(rastersize, rastersize);
    const coord = range(-rastersize/2, rastersize/2);

    let m_orders = [];
    for (let m = 0; m < m_range; m += 1) {
        if (m===0) {
            m_orders.push(m)
        }
        else{
            m_orders.push(  m );
            m_orders.push( -m );
        }
    }

    const pitch = rise*frequency;
    const midpoint = rastersize/2

    let n;
    let m;
    let z_line
    for (n = 0; n < n_range; n += 1) {
        for ( m of m_orders ) {
            console.log(`Computing solution for n = ${n} and m = ${m}`);
            z_line = Math.round( n / (pitch*scale) + m / (rise*scale));
            //console.log( 'zline is: '+ z_line);

            let bessel = zeros(rastersize);
            let i = 0;
            for (i = 0; i < rastersize; i += 1) {
                bessel = subset(bessel, index(i), besselj(2*Math.PI*subset(coord,index(i))*radius*scale, n)  )
            }

            let Ufac = multiply(bessel, exp(complex(0, n * Math.PI/2)));
            if (z_line <= rastersize/2-1 && z_line > -rastersize/2+1) {

                let old_zline = subset(raw_image, index(z_line + midpoint, range(0, rastersize)));
                raw_image = subset(raw_image, index(z_line + midpoint, range(0, rastersize)), add(squeeze(old_zline), Ufac)); //
                //old_zline = subset(raw_image, index(z_line + midpoint, range(0, rastersize)));
                //console.log(old_zline);
                if (n !== 0) {
                    // mirrored FFT reflection
                    old_zline = subset(raw_image, index(-z_line + midpoint, range(0, rastersize)));
                    raw_image = subset(raw_image, index(-z_line + midpoint, range(0, rastersize)), add(squeeze(old_zline), Ufac)); //
                }
            }
        }
    }

    const image = square(abs(raw_image));
    console.log('Analytical FFT Ready');

    return image //multiply(identity(rastersize), 128);
}

export function toImageArray(data, ImageData, maximum=-1){
    // we have 4 channels in imagedata (r,g,b,a)
    const numel = ImageData.data.length;

    const dataLen = data.size()[0]*data.size()[1];

    let max_data
    if (maximum < 0) {
        max_data = max(data);
    } else {
        max_data = maximum
    }
    const min_data = min(data);
    const rescaled_data = multiply(subtract(data, min_data), 255/(max_data-min_data)).reshape([dataLen]);  // Uint8

    let i;
    let data_val;

    // maybe move to math.js ranges? faster?
    let j = 0;
    for (i = 0; i < numel; i += 4) {
        data_val = subset(rescaled_data, index(j)) ;
        j++;
        ImageData.data[i] = data_val;
        ImageData.data[i+1] = data_val  ;
        ImageData.data[i+2] = data_val;
        ImageData.data[i+3] = 255;
    }
    console.log('finished conversion');
    return ImageData
}

export function toImageArray2(data,size=512 , maximum=-1){
    // we have 4 channels in imagedata (r,g,b,a)
    console.log(data.size()[0] )
    const dataLen = data.size()[0] * data.size()[1];

    let max_data;
    if (maximum < 0) {  // if set to default it will just take max of data instead of specified value
        max_data = max(data);
    } else {
        max_data = maximum
    }
    const min_data = min(data);
    const rescaled_data = multiply(subtract(data, min_data), 255/(max_data-min_data)).reshape([dataLen]);  // Uint8

    let i;
    let data_val;

    let ImageData = new Uint8Array( 4 * dataLen );

    // maybe move to math.js ranges? faster?
    let j = 0;
    for (i = 0; i < ImageData.length; i += 4) {
        data_val = subset(rescaled_data, index(j)) ;
        j++;
        ImageData[i]   = data_val;
        ImageData[i+1] = data_val;
        ImageData[i+2] = data_val;
        ImageData[i+3] = 255;
    }

    console.log('finished conversion');
    return ImageData
}

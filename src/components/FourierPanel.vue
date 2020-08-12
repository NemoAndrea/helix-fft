<template>
    <div>
        <v-btn outlined color="black" @click="updateRaster()">Test plot</v-btn>
        <div class="rasterImage"></div>
    </div>

</template>

<script>
import { toImageArray, fft_analytic }  from '../utils/fft_tools.js'

    export default {
        name: "FourierPanel",
        props: {
            helixFamily: {
                type: Array,
            },
        },
        data: () => ({
            analyticFFT: '',
            canvas: '',
            ctx: '',
            rasterSize: 512
        }),
        methods:{
            updateRaster(){
                for (let helix of this.helixFamily) {  // STILL NEEDS TO BE IMPLEMENTED FOR MULTIPLE HELICES
                    this.analyticFFT = fft_analytic( helix['radius'], helix['rise'], helix['frequency'],
                        helix['unit_size'], this.rasterSize, 3, 1, 0.001 );

                }
                let idata = this.ctx.createImageData( this.rasterSize, this.rasterSize );
                toImageArray( this.analyticFFT, idata, 0.3 );
                this.ctx.putImageData( idata, 0, 0 );
                //
                let image = new Image();
                image.src = this.canvas.toDataURL(); // produces a PNG file
                let imgel = document.querySelector( '.rasterImage' ).appendChild( image );
                imgel.style.width = '100%';
            }
        },
        mounted() {
            this.canvas = document.createElement( 'canvas' );
            this.canvas.width = this.rasterSize;
            this.canvas.height = this.rasterSize;
            this.ctx = this.canvas.getContext( '2d' );
        }
    }
</script>

<style scoped>
    .rasterImage{
    }
</style>

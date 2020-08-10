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
            radius: {
                type: Number,
                default: -1
            },
            rise: {
                type: Number,
                default: -1
            },
            frequency: {
                type: Number,
                default: -1
            },
            unit_size: {
                type: Number,
                default: -1
            }
        },
        data: () => ({
            analyticFFT: '',
            canvas: '',
            ctx: '',
            rasterSize: 512
        }),
        methods:{
            updateRaster(){
                this.analyticFFT = fft_analytic(this.radius, this.rise, this.frequency, this.unit_size, this.rasterSize,
                     3, 1, 0.001);
                let idata = this.ctx.createImageData(this.rasterSize, this.rasterSize);
                let buffer = toImageArray(this.analyticFFT, idata, 0.3);
                console.log(buffer);
                this.ctx.putImageData(idata, 0, 0);
                //
                let image = new Image();
                image.src = this.canvas.toDataURL(); // produces a PNG file
                document.querySelector( '.rasterImage' ).appendChild(image);
            }
         },
        mounted() {
            this.canvas = document.createElement('canvas');
            this.canvas.width = this.rasterSize, this.canvas.height = this.rasterSize;
            this.ctx = this.canvas.getContext('2d');
            console.log(this.ctx)
        }
    }
</script>

<style scoped>

</style>

<template>
    <div class="ui-fft-panel-sub">
        <div class="card card-display elevation-3">

            <div class="card-title">Display Controls (FFT contrast)</div>
            <div>
                <v-range-slider
                        v-model="contrast['range']"
                        thumb-label
                        label="Range"
                        :max=255
                        :min=0
                        hide-details
                        class="align-center"
                        color="var(--primary)"
                        track-color="darkgrey"
                        @end="updateContrast"
                />
                <v-slider class="ma-0 pa-0"
                          thumb-label
                          label="Offset"
                          v-model="contrast['offset']"
                          :max=128
                          :min=-128
                          color="var(--primary)"
                          track-color="darkgrey"
                          @end="updateContrast"
                />
                <button @click="updateFFT_old" style="background: lightcyan; color: black; padding: 5px; margin: 5px">legacy FFT calculation</button>
            </div>
        </div>
        <div class="card card-fft elevation-3">
            <div class="fft-card-header">
                <div class="card-title">FFT of helix (analytic solution)</div>
                <div class="order-dropdown-container">
                    <div class="order-dropdown">
                        <v-select
                                v-model="n_order"
                                :items="n_order_list"
                                label="n"
                                color="var(--primary)"
                                outlined
                                dense
                                hide-details
                        />
                    </div>
                    <div class="order-dropdown">
                        <v-select
                                v-model="m_order"
                                :items="m_order_list"
                                label="m"
                                color="var(--primary)"
                                outlined
                                dense
                                hide-details
                        />
                    </div>
                </div>
            </div>
            <div class="rasterImage"></div>
        </div>

    </div>

</template>

<script>
    // eslint-disable-next-line no-unused-vars
import { toImageArray, toImageArray2, fft_analytic, toIntArr }  from '../utils/fft_tools.js'

    export default {
        name: "FourierPanel",
        props: {
            helixFamily: {
                type: Array,
            },
            updateCounter: {
                type: Number,
                default: 0,
            },
        },
        watch: {
            // eslint-disable-next-line no-unused-vars
            updateCounter: {
                handler: function () {
                    this.updateFFT()
                }
            },
        },
        data: () => ({
            analyticFFT: '',
            n_order: 6,
            n_order_list: [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16],  // allowed values for n
            m_order: 1,
            m_order_list: [1,2,3,4,5,6,7],   // allowed values for +- m
            canvas: '',
            ctx: '',
            rasterSize: 512,
            camera: '',
            container: '',
            scene: '',
            contrast: { 'range':[0, 255], 'contrast':1, 'offset':0 },
            imageData: [],
            imageDataTest: [],
            image: '',
            wasm: '',
            wasm_contrast: '',
            wasm_fft_analytic: ''
        }),
        methods: {
            updateFFT_old(){
                console.time('ana-fft');
                for (let helix of this.helixFamily) {  // STILL NEEDS TO BE IMPLEMENTED FOR MULTIPLE HELICES
                    this.analyticFFT = fft_analytic( helix['radius'], helix['rise'], helix['frequency'],
                        helix['unit_size'], this.rasterSize, this.n_order, this.m_order, 0.01 );
                    break  // for now, until analyticFFT is fixed
                }
                console.timeEnd('ana-fft');

                let idata = this.ctx.createImageData( this.rasterSize, this.rasterSize );

                toImageArray( this.analyticFFT, idata, 0.5 );

                // save the raw image data to a variable as reference for contrast adjustment
                this.imageData = idata.data;
                this.imageDataTest = toIntArr(this.imageData);

                // set and display image data
                this.ctx.putImageData( idata, 0, 0 );
                this.image.src = this.canvas.toDataURL(); // update the src of the existing image

                this.updateContrast();  // make sure to apply existing contrast
            },

            updateFFT(){
                let apple;
                console.time('FFT-analytic-wasm');  //
                apple = this.wasm_fft_analytic( this.helixFamily, this.n_order, this.m_order, 0.01, this.rasterSize);

                let newImageData = new ImageData(Uint8ClampedArray.from(apple), this.rasterSize, this.rasterSize);
                this.ctx.putImageData( newImageData, 0, 0 );
                this.image.src = this.canvas.toDataURL(); // produces a PNG file

                console.timeEnd('FFT-analytic-wasm');
                this.imageDataTest = apple
                this.updateContrast();  // make sure to apply existing contrast
            },

            updateContrast() {
                console.time('contrast-wasm');
                const newdata = this.wasm_contrast(this.imageDataTest, this.contrast['offset'] ,this.contrast['range'][0], this.contrast['range'][1]);

                // convert types
                let newDataClamped = Uint8ClampedArray.from(newdata);
                let newImageData = new ImageData(newDataClamped, this.rasterSize, this.rasterSize);

                this.ctx.putImageData( newImageData, 0, 0 );
                this.image.src = this.canvas.toDataURL(); // produces a PNG file
                console.timeEnd('contrast-wasm');
            },

            async loadWASMfuncs (){
                this.wasm_contrast = (await this.wasm).set_contrast;
                this.wasm_fft_analytic = (await this.wasm).FFT_analytic;
            },
        },
        mounted() {
            this.canvas = document.createElement( 'canvas' );
            this.canvas.width = this.rasterSize;
            this.canvas.height = this.rasterSize;
            this.ctx = this.canvas.getContext( '2d' );

            this.wasm = import("../../wasm/pkg");

            this.imageDataTest = new Uint8Array( 4 * this.rasterSize * this.rasterSize );

            this.loadWASMfuncs();

            // actually attach an image object to canvas.
            this.image = new Image();
            this.image.src = this.canvas.toDataURL(); // produces a PNG file
            document.querySelector( '.rasterImage' ).appendChild( this.image );

            console.log('[ Fourierpanel mounted ]')
        }
    }
</script>

<style scoped>
    .rasterImage{
        width: 100%;
    }

    img .rasterImage{
        height: 10rem;
    }

    .ui-fft-panel-sub {
        display: grid;
        grid-template-rows: auto 1fr ;
        height: 100%;
    }

    .fft-card-header{
        display: flex;
        align-items: center;
        justify-content: space-between;
        margin-bottom: 0.5rem;
    }

    .order-dropdown{
        width: 50%;
        margin-left: 1rem;
    }

    .order-dropdown-container{
        display: flex;
        width: 30%;
    }
</style>

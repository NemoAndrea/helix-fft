<template>
    <div class="ui-fft-panel-sub">
        <div class="card card-display">

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
                <button @click="update_plot_scale(true)" style="background: lightcyan; color: black; padding: 5px; margin: 5px">Auto Scale FFT plot</button>
            </div>
        </div>
        <div class="card card-fft">
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
                handler: function () { this.updateFFT( false) }
            },
            n_order: {
                handler: function () { this.updateFFT( false ) }
            },
            m_order: {
                handler: function () { this.updateFFT( false ) }
            },
        },
        data: () => ({
            analyticFFT: '',
            n_order: 5,
            n_order_list: [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16],  // allowed values for n
            m_order: 0,
            m_order_list: [0,1,2,3,4,5,6,7],   // allowed values for +- m
            canvas: '',
            ctx: '',
            rasterSize: 512,
            plot_scale: 0.01,
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
                this.update_plot_scale(false);
                console.time('ana-fft');
                for (let helix of this.helixFamily) {  // STILL NEEDS TO BE IMPLEMENTED FOR MULTIPLE HELICES
                    this.analyticFFT = fft_analytic( helix['radius'], helix['rise'], helix['frequency'],
                        helix['unit_size'], this.rasterSize, this.n_order+1, this.m_order+1, this.plot_scale );
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
                this.image.style.backgroundImage = "url(" + this.canvas.toDataURL() + ")";

                this.updateContrast();  // make sure to apply existing contrast
            },

            updateFFT( autoscale ){
                if (autoscale) { this.update_plot_scale(false) }

                let FFT_image;
                console.time('FFT-analytic-wasm');  //
                FFT_image = this.wasm_fft_analytic( this.helixFamily, this.n_order+1, this.m_order+1,
                    this.plot_scale, this.rasterSize);

                let newImageData = new ImageData(Uint8ClampedArray.from(FFT_image), this.rasterSize, this.rasterSize);
                this.ctx.putImageData( newImageData, 0, 0 );

                this.image.style.backgroundImage = "url(" + this.canvas.toDataURL() + ")";
                console.timeEnd('FFT-analytic-wasm');
                this.imageDataTest = FFT_image;
                this.updateContrast();  // make sure to apply existing contrast
            },

            // set the plot scale such that the final layerline (n) is drawn at 70% of the frame height.
            update_plot_scale(redraw){
                const max_z_line = this.n_order / (this.helixFamily[0]['rise']*this.helixFamily[0]['frequency']*this.plot_scale);
                this.plot_scale  = max_z_line/(this.rasterSize/2*0.7) * this.plot_scale;
                if (redraw) { this.updateFFT(); }
            },

            updateContrast() {
                console.time('contrast-wasm');
                const newdata = this.wasm_contrast(this.imageDataTest, this.contrast['offset'] ,this.contrast['range'][0], this.contrast['range'][1]);

                // convert types
                let newDataClamped = Uint8ClampedArray.from(newdata);
                let newImageData = new ImageData(newDataClamped, this.rasterSize, this.rasterSize);

                this.ctx.putImageData( newImageData, 0, 0 );
                this.image.style.backgroundImage = "url(" + this.canvas.toDataURL() + ")";

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

            this.image = document.querySelector( '.rasterImage' );
            this.image.style.backgroundImage = "url(" + this.canvas.toDataURL() + ")";
            console.log('[ Fourierpanel mounted ]')
        }
    }
</script>

<style scoped>

    .ui-fft-panel-sub{
        height: 30rem;
    }

    .card-fft{
        border-radius: 9px;
        padding-bottom: 0.5rem;
        display: flex;
        flex-direction: column;
        height: 100%;
    }

    .rasterImage{
        flex-grow: 1;
        height:100%;
        max-height:100%;
        background-size: contain;
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

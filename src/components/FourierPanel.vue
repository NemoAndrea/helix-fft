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
                        @end="updateContrast"
                />
                <v-slider class="ma-0 pa-0"
                          thumb-label
                          label="Offset"
                          v-model="contrast['offset']"
                          :max=128
                          :min=-128
                          color="var(--primary)"
                          @end="updateContrast"
                />

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
import { toImageArray, toImageArray2, fft_analytic }  from '../utils/fft_tools.js'

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
            n_order_list: [1,2,3,4,5,6,7,8,9,10,11],  // allowed values for n
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
            image: ''
        }),
        methods: {
            updateFFT(){
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

                // set and display image data
                this.ctx.putImageData( idata, 0, 0 );
                this.image.src = this.canvas.toDataURL(); // update the src of the existing image
            },


            // it might be good to eventually move this to webassembly; runtime is about 150ms in js.
            updateContrast() {
                console.log('New contrast set!')

                console.time('contrast');
                let idata = this.ctx.getImageData( 0,0,this.rasterSize, this.rasterSize );
                let data = idata.data;

                // map the old image to new values
                let valRange = 255 / ( this.contrast['range'][1] - this.contrast['range'][0] );
                for (let j = 0; j < this.imageData.length; j+=4) {
                    data[j]   =  (this.imageData[j]   - this.contrast['range'][0]) * valRange + this.contrast['offset'];
                    data[j+1] =  (this.imageData[j+1] - this.contrast['range'][0]) * valRange + this.contrast['offset'];
                    data[j+2] =  (this.imageData[j+2] - this.contrast['range'][0]) * valRange + this.contrast['offset'];
                    // we do not change the alpha
                }

                this.ctx.putImageData( idata, 0, 0 );
                this.image.src = this.canvas.toDataURL(); // produces a PNG file
                console.timeEnd('contrast');
            },
        },
        mounted() {
            this.canvas = document.createElement( 'canvas' );
            this.canvas.width = this.rasterSize;
            this.canvas.height = this.rasterSize;
            this.ctx = this.canvas.getContext( '2d' );

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

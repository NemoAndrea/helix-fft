<template>
    <div class="ui-fft-panel-sub">
        <div class="card card-display">
            <div class="display-controls-header">
                <div class="card-title">Diffraction Display Controls</div>
                <v-tooltip bottom >
                    <template v-slot:activator="{ on }">
                        <v-icon v-on="on" @click="update_plot_scale(true)"> mdi-arrow-split-horizontal</v-icon>
                    </template>
                    <span>resize diffraction plot to fit current n</span>
                </v-tooltip>
            </div>
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
            </div>
        </div>

        <div class="card card-fft">
            <div class="fft-card-header">
                <div class="fft-card-header-left">
                    <div class="card-title">Diffraction pattern (analytic)</div>
                    <div v-if="updateCounter>0"> <!--check we have an image. Not perfect but fine for most cases.-->
                    <v-tooltip bottom>
                        <template v-slot:activator="{ on }">
                            <a id="download-anchor">
                            <v-icon v-on="on" @click="download_fft()" class="download"> mdi-download </v-icon>
                            </a>
                        </template>
                        <span>Download FFT image</span>
                    </v-tooltip>
                    </div>
                </div>
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
                                @change="updateFFT( false )"
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
                                @change="updateFFT( false )"
                        />
                    </div>
                </div>
            </div>
            <div class="rasterImage"></div>
        </div>

    </div>

</template>

<script>
    import Panzoom from '@panzoom/panzoom'

    export default {
        name: "FourierPanel",
        props: {
            helixFamily: {
                type: Array,
            },
            name: {
                type: String,
            },
            updateCounter: {
                type: Number,
                default: 0,
            },
            externalDisplayParams: {
                type: Object
            },
        },
        watch: {
            // eslint-disable-next-line no-unused-vars
            updateCounter: {
                handler: function () { this.updateFFT( false) }
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
            updateFFT( autoscale ){
                if (autoscale) { this.update_plot_scale(false) }

                let FFT_image;
                console.time('FFT-analytic-wasm');  //
                FFT_image = this.wasm_fft_analytic( this.helixFamily, this.n_order+1, this.m_order+1,
                    this.plot_scale, this.rasterSize);

                let newImageData = new ImageData(Uint8ClampedArray.from(FFT_image), this.rasterSize, this.rasterSize);
                this.ctx.putImageData( newImageData, 0, 0 );

                this.image.src = this.canvas.toDataURL(); // produces a PNG file
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
                this.image.src = this.canvas.toDataURL(); // produces a PNG file

                console.timeEnd('contrast-wasm');
            },

            download_fft(){
                console.log('> exporting FFT image');
                let anchor = document.getElementById("download-anchor");
                let currentDate = new Date().toISOString().slice(0,10);
                anchor.setAttribute('href', this.canvas.toDataURL()); // set image data to current displayed image
                const filename = `${currentDate}_FFT_${this.name}`;
                anchor.setAttribute('download', filename); // set download name
            },

            // export display parameters so that they can be added to the model URL
            exportDisplayParams(){
                let exportParams = {};  // we will add n,m, plot_scale if they are not the default value.

                if (this.n_order !== 5){
                    exportParams['n'] = this.n_order
                }
                if (this.m_order !== 0){
                    exportParams['m'] = this.m_order
                }
                if (this.plot_scale !== 0.01) {
                    // we scale the plot_scale to be more compact (and rescale when importing)
                    // and round it to keep URL readable.
                    exportParams['s'] = +(this.plot_scale * 100).toFixed(2);
                }

                // export non-default values
                this.$emit( 'exportDisplayParams', exportParams )
            },

            // the URL with the model contained display parameters, so we should set them, overriding the default
            setDisplayParams (){
                for (const [key, value] of Object.entries( this.externalDisplayParams ) ) {
                    if (key === 'n') {
                        if ( this.n_order_list.includes(Number(value)) ){  // make sure we are considering a valid value
                            this.n_order = Number(value);
                        }
                    } else if (key == 'm'){
                        if ( this.m_order_list.includes(Number(value)) ){  // make sure we are considering a valid value
                            this.m_order = Number(value);
                        }
                    } else if (key == 's'){
                        if (value > 0) {  // only positive values for the scale
                            this.plot_scale = value / 100;  // we have to scale it back ( see exportDisplayParams() )
                        }
                    }
                }
            },

            async loadWASMfuncs (){
                this.wasm_contrast = (await this.wasm).set_contrast;
                this.wasm_fft_analytic = (await this.wasm).WASM_diffraction_analytic;

            },
        },
        async mounted() {
            this.canvas = document.createElement( 'canvas' );
            this.canvas.width = this.rasterSize;
            this.canvas.height = this.rasterSize;
            this.ctx = this.canvas.getContext( '2d' );

            this.wasm = import("../../wasm/pkg");

            this.imageDataTest = new Uint8Array( 4 * this.rasterSize * this.rasterSize );

            this.image = new Image();
            this.image.src = this.canvas.toDataURL(); // produces a PNG file
            document.querySelector( '.rasterImage' ).appendChild( this.image );
            this.image.style.position = 'absolute';
            this.image.style.width = '100%';
            this.image.style.height = '100%';
            this.image.style.objectFit = 'contain';

            const panzoom = Panzoom(this.image, {
                minScale: 1,
            });
            this.image.parentElement.addEventListener('wheel', panzoom.zoomWithWheel);

            await this.loadWASMfuncs();
            this.setDisplayParams();  // set the external display paramters (loaded from URL)

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

    .card-display{
        padding-bottom: 0;
    }

    .display-controls-header{
        display: flex;
        justify-content: space-between;
        margin-bottom: 0.5rem;
    }

    .display-controls-header .card-title {
        margin-bottom: 0;
    }

    .rasterImage{
        flex-grow: 1;
        height:100%;
        max-height:100%;
        position: relative;
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

    .fft-card-header-left{
        display: flex;
    }

    .order-dropdown{
        width: 50%;
        margin-left: 1rem;
    }

    .order-dropdown-container{
        display: flex;
        width: 30%;
    }

    .download{
        margin: 4px;
    }

    @media only screen and (max-width: 600px) {
        .card-fft{
            height:120vw;
            padding: 0;
        }

        .order-dropdown{
            width: 100%;
            margin-left: 0;
            margin-bottom: 0.5rem;
        }

        .order-dropdown-container{
            display: block;
            width: 30%;
        }

        .fft-card-header{
            margin: 0.5rem 1.5rem 0 1.5rem;
        }
        .rasterImage{
            margin: 0.5rem;
        }
    }
</style>

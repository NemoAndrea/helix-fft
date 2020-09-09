<template>
    <div class="ui-fft-panel-sub">
        <div class="card card-display">
            <div class="display-controls-header">
                <div class="card-title">Diffraction Display Controls</div>

                <div class="display-controls-drawer">
                    <!--LUT manager -->
                    <LutManager :context2D="ctx" ref="LUTs"
                                :img="image"
                                :max="LUT_settings['range'][1]/255"
                                :min="LUT_settings['range'][0]/255"
                                :offset="LUT_settings['offset']"
                                :gamma="LUT_settings['gamma']"
                                v-on:lut_update="zoomDiffractionPlot(lastTransform)"/>
                    <!-- USER IMAGE UPLOAD-->
                    <v-menu left bottom  transition="slide-y-transition" :offset-y=true
                            :close-on-content-click=false>
                        <template v-slot:activator="{ on: menu, attrs }">
                            <v-tooltip bottom >
                                <template v-slot:activator="{ on: tooltip }">
                                    <v-btn v-bind="attrs" icon v-on="{ ...tooltip, ...menu }"
                                           class="display-controls-button">
                                        <v-icon > mdi-microscope </v-icon>
                                    </v-btn>
                                </template>
                                <span> Load experimental helix image </span>
                            </v-tooltip>
                        </template>
                        <v-list shaped class="upload-window">
                            <v-subheader>UPLOAD EXPERIMENTAL IMAGE</v-subheader>
                            <v-list-item><v-alert type="error">
                                Only 512x512 images are currently supported!
                            </v-alert></v-list-item>
                            <v-list-item ripple>
                                <div><v-file-input
                                        :rules="uploadRules"
                                        filled
                                        accept="image/png, image/jpeg, image/bmp"
                                        placeholder="Upload a local image"
                                        prepend-icon="mdi-camera"
                                        v-model="imageUpload"
                                        color="var(--primary)"
                                        @change="show_preview"
                                ></v-file-input>
                                    <img class="upload-preview">
                                </div>
                            </v-list-item>
                            <span v-if="imageUpload!==null">
                            <v-list-item ripple> <div>
                                <div class="upload-menu-header">What kind of image is it?</div>
                                <v-radio-group dense v-model="upload_needs_fourier" class="compact-radio">
                                    <v-radio label="Realspace" :value=true color="var(--primary)"></v-radio>
                                    <v-radio label="Diffraction Pattern" :value=false color="var(--primary)"></v-radio>
                                </v-radio-group></div>
                             </v-list-item> </span>
                            <span v-if="upload_needs_fourier!==null"><v-list-item ripple>
                                    <div><div class="upload-menu-header">Specify pixel size</div>
                                <v-text-field label="Pixel size"
                                              :rules="numberRules"
                                              suffix="nm"
                                              color="var(--primary)"
                                              type="number" v-model="uploadScale"></v-text-field></div>
                            </v-list-item></span>
                            <span v-if="uploadScale > 0"><v-list-item ripple>
                                <v-btn v-show="upload_needs_fourier===true" color="var(--primary)" @click="process_upload()">compute FFT and show</v-btn>
                                <v-btn v-show="upload_needs_fourier===false" color="var(--primary)" @click="process_upload()">show</v-btn>
                            </v-list-item></span>

                        </v-list>
                    </v-menu>
                    <!-- DISTANCE MEASUREMENT SETTINGS-->
                    <v-menu left bottom :close-on-click=true transition="slide-y-transition" :offset-y=true >
                        <template v-slot:activator="{ on: menu, attrs }">
                            <v-tooltip bottom>
                                <template v-slot:activator="{ on: tooltip }">
                                    <v-btn v-bind="attrs" icon v-on="{ ...tooltip, ...menu }" class="display-controls-button">
                                        <v-icon > mdi-tape-measure </v-icon>
                                    </v-btn>
                                </template>
                                <span> Distance measurement settings </span>
                            </v-tooltip>
                        </template>
                        <v-list shaped>
                            <v-subheader>OVERLAY SETTINGS</v-subheader>
                            <v-list-item ripple>
                                <v-checkbox
                                        v-model="coordinates['hidden']"
                                        label="Hide distances"
                                ></v-checkbox>
                            </v-list-item>
                            <v-subheader>DISPLAY STYLE</v-subheader>
                            <v-list-item ripple>
                                <v-radio-group v-model="coordinates_as_frequency">
                                    <v-radio label="frequency [1/nm]" :value="true"></v-radio>
                                    <v-radio label="wavelength (1/f)" :value="false"></v-radio>
                                </v-radio-group>
                            </v-list-item>
                        </v-list>
                    </v-menu>

                    <!--                <v-tooltip bottom >-->
                    <!--                    <template v-slot:activator="{ on }">-->
                    <!--                        <v-icon v-on="on" class="display-controls-button"> mdi-currency-eth </v-icon>-->
                    <!--                    </template>-->
                    <!--                    <span> Toggle overlay first zeros of analytic solution</span>-->
                    <!--                </v-tooltip>-->
                    <!-- SET PLOT SCALE -->
                    <v-tooltip bottom >
                        <template v-slot:activator="{ on }">
                            <v-icon v-on="on" @click="update_plot_scale(true)"
                                    class="display-controls-button"> mdi-arrow-split-horizontal</v-icon>
                        </template>
                        <span>resize diffraction plot to fit current n</span>
                    </v-tooltip>
                </div>
            </div>
            <div>
                <v-range-slider
                        v-model="LUT_settings['range']"
                        thumb-label
                        label="Range"
                        :max=255
                        :min=0
                        hide-details
                        class="align-center"
                        color="var(--primary)"
                        track-color="darkgrey"
                />
                <div class="side-by-side-slider">
                    <v-slider class="ma-0 pa-0"
                                thumb-label
                                label="Offset"
                                v-model="LUT_settings['offset']"
                                :max=1
                                :min=-1
                                :step=0.05
                                color="var(--primary)"
                                track-color="darkgrey"
                    />
                    <v-slider class="ma-0 pa-0"
                              thumb-label
                              label="Gamma"
                              v-model="LUT_settings['gamma']"
                              :max=5
                              :min=0.1
                              :step=0.1
                              color="var(--primary)"
                              track-color="darkgrey"
                    />
                </div>
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
            <div style="margin: 0.5rem"><div class="image-container">
                <canvas class="rasterImage"></canvas>
                <!--style="background-color: red; opacity: 30%"-->
                <svg class="rasterImageOverlay"  ></svg>
                <div v-show="!coordinates['hidden'] && this.updateCounter > 0" class="coordinates-overlay">
                    <div v-if="coordinates_as_frequency"> f: {{coordinates['d'].toFixed(2)}} 1/nm (z: {{coordinates['z'].toFixed(2)}} 1/nm, r: {{coordinates['r'].toFixed(2)}} 1/nm) </div>
                    <div v-if="!coordinates_as_frequency"> 1/f: {{coordinates['d'].toFixed(2)}} nm (z: {{coordinates['z'].toFixed(2)}} nm, r: {{coordinates['r'].toFixed(2)}} nm)</div>
                </div>
            </div></div>
        </div>

    </div>
</template>

<script>
    import LutManager from "./LutManager";
    import * as d3 from "d3";
    import { upload_to_rgba } from "../utils/upload_utils";

    export default {
        name: "FourierPanel",
        components: {LutManager},
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
            n_order_list: [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25],  // allowed values for n
            m_order: 0,
            m_order_list: [0,1,2,3,4,5,6,7],   // allowed values for +- m
            canvas: null,
            overlay: null,
            ctx: null,
            rasterSize: 512,
            plot_scale: 0.01,
            camera: '',
            scene: '',
            LUT_settings: { 'range':[0, 255], 'offset':0, 'gamma': 1 },
            imageData: [],
            imageDataTest: [],
            image: null,
            wasm: '',
            wasm_fft_analytic: '',
            wasm_fft: '',
            coordinates_as_frequency: false,
            coordinates: {'d': 0, 'z': 0, 'r': 0, 'hidden': false},
            uploadRules: [
                value => !value || value.size < 2000000 || 'Image size should be less than 2 MB!',
            ],
            imageUpload: null,
            upload_needs_fourier: null,
            uploadScale: 0,
            numberRules: [
                v => !!v || 'parameter is required',
                v => v > 0 || 'value must be larger than 0',
            ],
            lastTransform: null,
        }),
        methods: {
            updateFFT( autoscale ){
                if (autoscale) { this.update_plot_scale(false) }

                let FFT_image;
                console.time('FFT-analytic-wasm');  //
                FFT_image = this.wasm_fft_analytic( this.helixFamily, this.n_order, this.m_order,
                    this.plot_scale, this.rasterSize );

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

            updateContrast() {  // manual application of LUTs. Should be called when new image is loaded.
                console.log('refreshing image contrast');
                this.$nextTick(() => {
                    // This code runs after the DOM has been updated.
                    //  we need to wait for the DOM to be updated (after we set a new image) before we can apply LUTs
                    this.$refs.LUTs.updateImage()
                });
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


            // this function still needs a lot of work, it needs to be able to handle arbitrary dimensions
            // and adjust the plot based on the pixel size
            process_upload(){
                console.log('Processing Uploaded image');
                const reader = new FileReader();

                reader.readAsDataURL(this.imageUpload);
                reader.onload = () => {
                    const rgba = upload_to_rgba(reader.result);
                    let newImageData;

                    // if we have realspace image, we need to take the FFT and show that
                    // if it is a diffraction pattern, we can just display the image straight away
                    if (this.upload_needs_fourier) {
                        console.time('wasm-FFT');
                        let FFT = this.wasm_fft(rgba);
                        console.timeEnd('wasm-FFT');

                        // convert types
                        newImageData = new ImageData(Uint8ClampedArray.from(FFT), this.rasterSize, this.rasterSize);
                    }
                    else {  // we dont need to compute FFT, so we just cast our image to the same type
                        // convert types
                        newImageData = new ImageData( Uint8ClampedArray.from(rgba), this.rasterSize, this.rasterSize);
                    }

                    this.ctx.putImageData( newImageData, 0, 0 );
                    this.image.src = this.canvas.toDataURL();
                    this.updateContrast();
                };

            },

            updateMouseCoordinate( event ){
                const pointer = d3.pointer(event);
                const x = ( Math.round(pointer[0]) ) * this.plot_scale;
                const y = ( Math.round(pointer[1]) ) * this.plot_scale;


                if ( this.coordinates_as_frequency ) {  // display as values with units 1/distance (Frequency)
                    this.coordinates['d'] = Math.round( Math.sqrt(x**2 + y**2) * 100)/100;
                    this.coordinates['z'] =  Math.abs(Math.round( y*100 ) / 100);
                    this.coordinates['r'] =  Math.abs(Math.round( x*100 ) / 100);
                }
                else {  //  vales are inverted to go from frequency to distance
                    this.coordinates['d'] = Math.round( 1 / Math.sqrt(x**2 + y**2) * 100)/100;
                    this.coordinates['z'] =  Math.abs(Math.round( (1/y)*100 ) / 100);
                    this.coordinates['r'] =  Math.abs(Math.round( (1/x)*100 ) / 100);
                }
            },

            zoomDiffractionPlot(transform){
                this.lastTransform = transform;
                const scale = transform.k;

                // for the .rasterImageOverlay (svg)
                this.overlay.attr("transform", transform);

                // for the .rasterImage (canvas)
                const newWidth  = this.rasterSize*scale;
                const newHeight = this.rasterSize*scale;
                this.ctx.save();
                this.ctx.clearRect(0, 0, this.rasterSize, this.rasterSize);
                this.ctx.translate( -((newWidth-this.rasterSize )/2) + transform.x,
                    -((newHeight-this.rasterSize)/2) + transform.y );
                this.ctx.scale(scale, scale);
                this.ctx.drawImage(this.image, 0, 0, this.rasterSize, this.rasterSize);  // draw diffraction image
                this.ctx.restore()
            },

            show_preview(){
                const reader = new FileReader();
                reader.addEventListener("load", function () {
                    // convert image file to base64 string
                    document.querySelector('.upload-preview').src = reader.result;
                }, false);

                if (this.imageUpload) {
                    reader.readAsDataURL(this.imageUpload);
                }
            },

            async loadWASMfuncs (){
                this.wasm_fft_analytic = (await this.wasm).wasm_diffraction_analytic;
                this.wasm_fft = (await this.wasm).wasm_FFT;
            }
        },
        async mounted() {
            this.canvas = document.querySelector( '.rasterImage' );
            this.canvas.width = this.rasterSize;
            this.canvas.height = this.rasterSize;
            this.ctx = this.canvas.getContext( '2d' );
            this.ctx.imageSmoothingEnabled = false;  // ensure we have clear pixelation (no smoothing)

            this.wasm = import("../../wasm/pkg");

            this.imageDataTest = new Uint8Array( 4 * this.rasterSize * this.rasterSize );

            this.image = new Image();
            this.image.src = this.canvas.toDataURL(); // produces a PNG file
            this.ctx.drawImage(this.image, 0, 0, 512, 512);  // draw diffraction image

            await this.loadWASMfuncs();
            this.setDisplayParams();  // set the external display paramters (loaded from URL)

            console.log('[ Fourierpanel mounted ]');

            // setup the SVG overlay (using D3)
            this.overlay = d3.select(".rasterImageOverlay")
                .attr("viewBox", [-this.rasterSize/2, -this.rasterSize/2, this.rasterSize, this.rasterSize])
                .on("mousemove", (event) => { this.updateMouseCoordinate(event) })
                .call(d3.zoom()
                    .extent([[-this.rasterSize/2,-this.rasterSize/2],[this.rasterSize/2-1, this.rasterSize/2-1]])
                    .scaleExtent([1, 5])
                    .translateExtent([[-this.rasterSize/2, -this.rasterSize/2], [this.rasterSize/2-1, this.rasterSize/2-1]])
                    .on("zoom", ({transform}) => this.zoomDiffractionPlot(transform)));
            this.overlay.node();  // update the SVG
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

    .image-container{
        position: relative;
        width: 100%;
        height: 100%;
        overflow: hidden;
    }

    .rasterImage{
        max-width: 100%;
        max-height: 100%;

        width: inherit;;
        position: absolute;
    }

    .rasterImageOverlay{
        max-width: 100%;
        max-height: 100%;

        position: relative;
        width: inherit;
        cursor: crosshair;
    }

    .coordinates-overlay{
        color:  var(--primary);
        position: absolute;
        top: 10px;
        left: 10px;

    }

    .ui-fft-panel-sub {
        display: grid;
        grid-template-rows: auto 1fr ;
        height: 100%;
    }

    .display-controls-button{
        margin-left: 1.2rem;
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

    .display-controls-drawer{
        display: flex;
        align-items: center;
    }

    .download{
        margin: 4px;
    }

    .upload-window{
        width: 22rem;
    }

    .upload-preview{
        width: 100%;
        display: block;
    }

    .upload-menu-header{
        color: var(--primary);
        font-weight: bold;
        margin-top: 0.2rem;
    }

    .compact-radio{
        margin: 0.1rem;
    }

    .side-by-side-slider{
        display: flex;
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

        .upload-window{
            width: auto;
        }
    }
</style>

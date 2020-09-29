<template>
    <div class="ui-fft-panel-sub">
        <div class="card card-display">
            <div class="display-controls-header">
                <div class="card-title">Adjusting
                    <span v-show="current_LUT_image === ImageType.ANALYTIC">
                        <v-tooltip top>
                            <template v-slot:activator="{ on }">
                                    <span class="current_LUT_image highlight primarycol" v-on="on"
                                          @click="current_LUT_image = ImageType.EXPERIMENTAL">
                                        Analytic image
                                    </span>
                            </template>
                            <span>Switch to experimental image controls</span>
                        </v-tooltip>
                    </span>
                    <span v-show="current_LUT_image === ImageType.EXPERIMENTAL">
                        <v-tooltip top>
                            <template v-slot:activator="{ on }">
                                    <span class="current_LUT_image highlight secondarycol" v-on="on"
                                          @click="current_LUT_image = ImageType.ANALYTIC">
                                        Experimental image
                                    </span>
                            </template>
                            <span>Switch to analytic image controls</span>
                        </v-tooltip>
                    </span>
                </div>
                <div class="display-controls-drawer">
                    <!--LUT manager -->
                    <LutManager :max_ana="LUT_settings_ana['range'][1]/255"
                                :min_ana="LUT_settings_ana['range'][0]/255"
                                :offset_ana="LUT_settings_ana['offset']"
                                :gamma_ana="LUT_settings_ana['gamma']"
                                :max_upl="LUT_settings_upl['range'][1]/255"
                                :min_upl="LUT_settings_upl['range'][0]/255"
                                :offset_upl="LUT_settings_upl['offset']"
                                :gamma_upl="LUT_settings_upl['gamma']"
                                :current_image = current_LUT_image
                                v-on:new_filter_string="filterStrings = arguments[0]"
                                v-on:lut_update="zoomDiffractionPlot(lastTransform)"
                                ref="LUTmanager"/>
                    <!-- DISTANCE MEASUREMENT SETTINGS-->
                    <v-menu left bottom :close-on-click=true transition="slide-y-transition" :offset-y=true >
                        <template v-slot:activator="{ on: menu, attrs }">
                            <v-tooltip bottom>
                                <template v-slot:activator="{ on: tooltip }">
                                    <v-btn v-bind="attrs" color="var(--bw-button-color)" icon
                                           v-on="{ ...tooltip, ...menu }" class="display-controls-button">
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
                                    <v-radio color="var(--primary)" label="frequency [1/nm]" :value="true"></v-radio>
                                    <v-radio color="var(--primary)" label="wavelength (1/f)" :value="false"></v-radio>
                                </v-radio-group>
                            </v-list-item>
                        </v-list>
                    </v-menu>

                    <v-tooltip bottom >
                        <template v-slot:activator="{ on }">
                            <v-btn
                                    icon color="var(--bw-button-color)"
                                    @click="toggle_maxima_overlay"  v-on="on"
                                    class="display-controls-button" >
                                <v-icon > mdi-altimeter </v-icon>
                            </v-btn>
                        </template>
                        <span> Toggle overlay of bessel functions first maxima </span>
                    </v-tooltip>
                    <!-- SET PLOT SCALE -->
                    <v-tooltip bottom >
                        <template v-slot:activator="{ on }">
                            <v-icon v-on="on" color="var(--bw-button-color)" @click="update_plot_scale(true)"
                                    class="display-controls-button"> mdi-arrow-split-horizontal</v-icon>
                        </template>
                        <span>resize diffraction plot to fit current n</span>
                    </v-tooltip>
                    <!-- USER IMAGE UPLOAD-->
                    <v-dialog v-model="show_upload_dialog" persistent max-width="44rem">
                        <template v-slot:activator="{ on: menu, attrs }">
                            <v-tooltip bottom >
                                <template v-slot:activator="{ on: tooltip }">
                                    <v-btn v-bind="attrs" v-on="{ ...tooltip, ...menu }" icon
                                           class="display-controls-button" color="var(--bw-button-color)">
                                        <v-icon> mdi-microscope </v-icon>
                                    </v-btn>
                                </template>
                                <span> Upload experimental helix image </span>
                            </v-tooltip>
                        </template>
                        <div class="upload-window">
                            <div class="upload-menu-title">
                                <v-subheader>UPLOAD EXPERIMENTAL IMAGE</v-subheader>
                                <v-btn icon color="var(--primary)" @click="show_upload_dialog=false"
                                       class="upload-close-mobile"><v-icon>mdi-close-circle-outline</v-icon></v-btn>
                            </div>
                            <img class="upload-preview" >
                            <div class="upload-input"><v-file-input
                                    :rules="uploadRules"
                                    filled
                                    accept="image/png, image/jpeg, image/bmp"
                                    placeholder="Upload a local image"
                                    prepend-icon="mdi-camera"
                                    v-model="imageUpload"
                                    color="var(--primary)"
                                    @change="show_preview"
                            ></v-file-input>

                                <span v-if="imageUpload!==null">

                                <div class="upload-menu-header">What kind of image is it?</div>
                                <v-radio-group dense v-model="upload_needs_fourier" class="compact-radio">
                                    <v-radio label="Realspace" :value=true color="var(--primary)"></v-radio>
                                    <v-radio label="Diffraction Pattern" :value=false color="var(--primary)"></v-radio>
                                </v-radio-group>
                                </span>
                                <span v-if="upload_needs_fourier!==null">
                                    <div><div class="upload-menu-header">Specify pixel size</div>
                                <v-text-field label="Pixel size"
                                              :rules="numberRules"
                                              suffix="nm"
                                              color="var(--primary)"
                                              type="number" v-model="uploadScale"
                                              autofocus></v-text-field></div>
                                </span>
                                <span v-if="uploadScale > 0">
                                <v-btn v-show="upload_needs_fourier===true" color="var(--primary)" @click="process_upload()">compute FFT and show</v-btn>
                                <v-btn v-show="upload_needs_fourier===false" color="var(--primary)" @click="process_upload()">show</v-btn>
                                </span>

                            </div>
                            <v-btn color="var(--primary)"
                                   @click="show_upload_dialog=false" outlined class="upload-close">close</v-btn>
                        </div>
                    </v-dialog>
                </div>
            </div>
            <transition name="fade">
                <div class="lut_controls_analytic" v-show="current_LUT_image === ImageType.ANALYTIC">
                    <div class="side-by-side">
                    <v-range-slider
                            v-model="LUT_settings_ana['range']"
                            thumb-label
                            label="Range"
                            :max=255
                            :min=0
                            hide-details
                            class="align-center"
                            color="var(--primary)"
                            track-color="var(--bw-slider-track-color)"
                    />
                    <v-tooltip bottom><template v-slot:activator="{ on }"> <v-btn v-show="visible['analytic']" icon color="var(--primary)" @click="visible['analytic']=false"  class="hide-button" v-on="on">
                        <v-icon>mdi-eye</v-icon>
                    </v-btn></template>
                        <span>Hide analytic image</span></v-tooltip>
                    <v-tooltip bottom><template v-slot:activator="{ on }"> <v-btn v-show="!visible['analytic']" icon color="var(--primary)" @click="visible['analytic']=true"  class="hide-button" v-on="on">
                        <v-icon>mdi-eye-off</v-icon>
                    </v-btn></template>
                        <span>Show analytic image</span></v-tooltip>
                    </div>
                    <div class="side-by-side-slider">
                        <v-slider class="ma-0 pa-0"
                                  thumb-label
                                  label="Offset"
                                  v-model="LUT_settings_ana['offset']"
                                  :max=1
                                  :min=-1
                                  :step=0.05
                                  color="var(--primary)"
                                  track-color="var(--bw-slider-track-color)"
                                  hide-details
                        />
                        <v-slider class="ma-0 pa-0"
                                  thumb-label
                                  label="Gamma"
                                  v-model="LUT_settings_ana['gamma']"
                                  :max=5
                                  :min=0.1
                                  :step=0.1
                                  color="var(--primary)"
                                  track-color="var(--bw-slider-track-color)"
                        />
                    </div>
                </div>
            </transition>
            <transition name="fade">
                <div class="lut_controls_upload" v-show="current_LUT_image === ImageType.EXPERIMENTAL">
                    <div class="side-by-side">
                    <v-range-slider
                            v-model="LUT_settings_upl['range']"
                            thumb-label
                            label="Range"
                            :max=255
                            :min=0
                            hide-details
                            class="align-center"
                            color="var(--secondary)"
                            track-color="var(--bw-slider-track-color)"
                    />
                    <v-tooltip bottom><template v-slot:activator="{ on }"> <v-btn v-show="visible['upload']" icon color="var(--secondary)" @click="visible['upload']=false"  v-on="on" class="hide-button">
                        <v-icon>mdi-eye</v-icon>
                    </v-btn></template>
                        <span>Hide experimental image</span></v-tooltip>
                    <v-tooltip bottom><template v-slot:activator="{ on }"> <v-btn v-show="!visible['upload']" icon color="var(--secondary)" @click="visible['upload']=true"  v-on="on" class="hide-button">
                        <v-icon>mdi-eye-off</v-icon>
                    </v-btn></template>
                        <span>Show experimental image</span></v-tooltip>
                    </div>
                    <div class="side-by-side-slider">
                        <v-slider class="ma-0 pa-0"
                                  thumb-label
                                  label="Offset"
                                  v-model="LUT_settings_upl['offset']"
                                  :max=1
                                  :min=-1
                                  :step=0.05
                                  color="var(--secondary)"
                                  track-color="var(--bw-slider-track-color)"
                                  hide-details
                        />
                        <v-slider class="ma-0 pa-0"
                                  thumb-label
                                  label="Gamma"
                                  v-model="LUT_settings_upl['gamma']"
                                  :max=5
                                  :min=0.1
                                  :step=0.1
                                  color="var(--secondary)"
                                  track-color="var(--bw-slider-track-color)"
                        />
                    </div>
                </div>
            </transition>


        </div>

        <div class="card card-fft">
            <div class="fft-card-header">
                <div class="fft-card-header-left">
                    <div class="card-title">Diffraction pattern</div>
                    <div v-if="updateCounter>0"> <!--check we have an image. Not perfect but fine for most cases.-->
                        <v-tooltip bottom>
                            <template v-slot:activator="{ on }">
                                <a id="download-anchor">
                                    <v-icon v-on="on" @click="download_fft()" class="download"> mdi-download </v-icon>
                                </a>
                            </template>
                            <span>Download Image</span>
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

                        /> <!--@change="updateFFT( false )"-->
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

            <div style="margin: 0.5rem"><v-responsive :aspect-ratio="1" class="image-container">
                <canvas class="rasterImage"></canvas>
                <!--style="background-color: red; opacity: 30%"-->
                <svg class="rasterImageOverlay"  ></svg>
                <div v-show="!coordinates['hidden'] && this.updateCounter > 0" class="coordinates-overlay">
                    <div v-if="coordinates_as_frequency"> f: {{coordinates['d'].toFixed(2)}} 1/nm (z: {{coordinates['z'].toFixed(2)}} 1/nm, r: {{coordinates['r'].toFixed(2)}} 1/nm) </div>
                    <div v-if="!coordinates_as_frequency"> 1/f: {{coordinates['d'].toFixed(2)}} nm (z: {{coordinates['z'].toFixed(2)}} nm, r: {{coordinates['r'].toFixed(2)}} nm)</div>
                </div>
            </v-responsive></div>

        </div>
        <v-snackbar v-model="snackbar">
            {{ snackText }}
            <template v-slot:action="{ attrs }">
                <v-btn
                        color="pink"
                        text
                        v-bind="attrs"
                        @click="snackbar = false">
                    Close
                </v-btn>
            </template>
        </v-snackbar>

    </div>
</template>

<script>
    import LutManager from "./LutManager";
    import * as d3 from "d3";
    import { upload_to_rgba, ImageData_to_dataURL , ImageType } from "../utils/upload_utils";
    import { draw_overlay } from "../utils/overlay_utils";

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
            visible: {  // make sure we also re-draw the image after toggling visibility.
                deep: true,
                handler: function () { this.zoomDiffractionPlot( this.lastTransform) }
            },
            n_order: {
                handler: function () { this.updateFFT( false ) }
            },
            m_order: {
                handler: function () { this.updateFFT( false ) }
            }
        },
        data: () => ({
            analyticFFT: null,
            n_order: 5,
            n_order_list: [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25],  // allowed values for n
            m_order: 0,
            m_order_list: [0,1,2,3,4,5,6,7],   // allowed values for +- m

            canvas: null,
            overlay: null,
            ctx: null,

            rasterSize: 512,
            displayFac: 1,
            plot_scale: 0.01,
            camera: null,
            scene: null,

            LUT_settings_ana: { 'range':[0, 255], 'offset':0, 'gamma': 1 },
            LUT_settings_upl: { 'range':[0, 255], 'offset':0, 'gamma': 1 },
            ImageType: ImageType,
            current_LUT_image: ImageType.ANALYTIC,  //  image we are applying LUT and contrast settings to (aa22222222222222222222222222222nalytic or upload)
            filterStrings: ['none', 'none'],
            imageData: [],
            imageDataTest: [],
            image: null,
            upload_image: null,
            upload_dim: {'width': 0, 'height': 0},  // we need to keep track of these, as they are not equal to rastersize
            visible: {'analytic': true, 'upload': true},  // whether to show or hide images (upload = experimental)
            show_maxima_overlay: false,

            wasm: null,
            wasm_fft_analytic: null,
            wasm_fft: null,
            wasm_bessel_max: null,

            coordinates_as_frequency: false,
            coordinates: {'d': 0, 'z': 0, 'r': 0, 'hidden': false},
            uploadRules: [
                value => !value || value.size < 2000000 || 'Image size should be less than 2 MB!',
            ],
            imageUpload: null,
            upload_needs_fourier: null,
            show_upload_dialog: false,
            uploadScale: 0,
            numberRules: [
                v => !!v || 'parameter is required',
                v => v > 0 || 'value must be larger than 0',
            ],
            lastTransform: d3.zoomIdentity,
            snackbar: false,
            snackText: ''
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
                this.refreshContrast();  // make sure to apply existing contrast
            },

            // set the plot scale such that the final layerline (n) is drawn at 70% of the frame height.
            update_plot_scale(redraw){
                const max_z_line = this.n_order / (this.helixFamily[0]['rise']*this.helixFamily[0]['frequency']*this.plot_scale);
                this.plot_scale  = max_z_line/(this.rasterSize/2*0.7) * this.plot_scale;
                if (redraw) { this.updateFFT(); }
            },

            refreshContrast() {  // manual application of LUTs. Should be called when new image is loaded.
                console.log('refreshing image contrast');
                this.$nextTick(() => {
                    // This code runs after the DOM has been updated.
                    //  we need to wait for the DOM to be updated (after we set a new image) before we can apply LUTs
                    this.zoomDiffractionPlot(this.lastTransform) // apply luts and zoom etc.

                    // if we have overlay on, we need to refresh it too
                    if (this.show_maxima_overlay)  { // overlay was being shown
                        this.toggle_maxima_overlay(); // turn off
                        this.toggle_maxima_overlay(); // turn on and redraw
                    }
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
                        if ( this.n_order_list.includes(+value) ){  // make sure we are considering a valid value
                            this.n_order = +value;
                        }
                    } else if (key == 'm'){
                        if ( this.m_order_list.includes(+value) ){  // make sure we are considering a valid value
                            this.m_order = +value;
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
                    try {
                        let raw_upload_image;
                        [ raw_upload_image, this.upload_dim.width, this.upload_dim.height] = upload_to_rgba(reader.result);
                        let newImageData;

                        // if we have realspace image, we need to take the FFT and show that
                        // if it is a diffraction pattern, we can just display the image straight away
                        if (this.upload_needs_fourier) {
                            console.time('wasm-FFT');
                            let FFT = this.wasm_fft(raw_upload_image.data, this.upload_dim.width, this.upload_dim.height);
                            console.timeEnd('wasm-FFT');
                            // unfortunately this is way we have to handle things for now
                            // until multi-value return becomes clearer for WASM.
                            this.upload_dim.width = FFT[FFT.length-2];
                            this.upload_dim.height = FFT[FFT.length-1];
                            // convert types
                            newImageData = new ImageData( Uint8ClampedArray.from(FFT.slice(0,-2)),
                                this.upload_dim.width, this.upload_dim.height);
                        }
                        else {  // we dont need to compute FFT, so we just cast our image to the same type
                            // convert types
                            newImageData = raw_upload_image //new ImageData( rgba , 512, 512);
                        }

                        this.upload_image.src = ImageData_to_dataURL(newImageData);
                        console.log('upload scale is: ', this.uploadScale);
                        this.refreshContrast();  // make sure we apply whatever contrast we had previously set
                        this.show_upload_dialog = false; // close upload dialog
                    }
                    catch (err) {  // some kind of error, but mainly for square. TODO: proper error handling
                        this.snackText = "Oops, something went wrong with uploaded image";
                        this.snackbar = true;
                        console.log(err)
                    }
                };
            },

            // calculate the spatial frequency that the cursor is placed at. (toggled under button 'measurement settings')
            updateMouseCoordinate( event ){
                const pointer = d3.pointer(event);
                // we get the physical coordinates. We draw this on the SVG, so we must correct with this.displayFac
                const x = ( Math.round(pointer[0]) - 0.5 ) * this.plot_scale * this.displayFac ;
                const y = ( Math.round(pointer[1]) - 0.5 ) * this.plot_scale * this.displayFac;

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
                this.ctx.translate( -((newWidth-this.rasterSize )/2) + transform.x*this.displayFac,
                    -((newHeight-this.rasterSize)/2) + transform.y*this.displayFac );
                this.ctx.scale(scale, scale);
                // draw analytic image
                if (this.visible['analytic']) {
                    this.ctx.filter = this.filterStrings[0];  // set LUT and contrast
                    this.ctx.drawImage(this.image, 0, 0, this.rasterSize, this.rasterSize);  // draw diffraction image
                }
                // draw experimental (uploaded) image
                if (this.visible['upload']) {
                    this.ctx.filter = this.filterStrings[1  ]; // set LUT and contrast
                    this.drawUpload();
                }
                this.ctx.restore()
            },

            drawUpload() {
                // we draw the uploaded image (diffraction plot) and we scale it appropriately
                // the overall scale incorporates pixel size and the plot scale for the analytic plot
                const scale_fac = 1/this.uploadScale*1/this.plot_scale*0.002;
                this.ctx.drawImage(this.upload_image,
                0, 0, this.upload_dim.width, this.upload_dim.height,
                this.rasterSize*(1-scale_fac)/2, this.rasterSize*(1-scale_fac)/2,
                this.rasterSize*scale_fac, this.rasterSize*scale_fac);
            },


            // function that overlays, for each bessel function, the location of the first maxima on the image.
            // this plot wll currently only take  information from the first helix in the helixfamily.
            toggle_maxima_overlay() {
                console.log(`Toggling maxima overlay (SVG) to ${!this.show_maxima_overlay}`);
                if (this.show_maxima_overlay){
                    this.show_maxima_overlay = false; // hide
                    document.querySelectorAll('.maxima').forEach(e => e.remove());
                } else {
                    this.show_maxima_overlay = true; // set internal state to shown
                    draw_overlay(this.overlay, this.wasm_bessel_max, this.helixFamily[0],
                        this.m_order, this.n_order_list, this.plot_scale * this.displayFac)
                }
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
                this.wasm_bessel_max = (await this.wasm).wasm_bessel_first_max;
            }
        },
        async mounted() {
            // intialise Canvas
            this.canvas = document.querySelector( '.rasterImage' );
            this.canvas.width = this.rasterSize;
            this.canvas.height = this.rasterSize;
            this.ctx = this.canvas.getContext( '2d' );
            this.ctx.imageSmoothingEnabled = false;  // ensure we have clear pixelation (no smoothing)
            this.ctx.globalCompositeOperation = 'lighten';  //additive colour blending


            this.imageDataTest = new Uint8Array( 4 * this.rasterSize * this.rasterSize );

            // initialise the Image objects
            this.image = new Image();
            this.upload_image = new Image();

            // load WebAssembly functions
            this.wasm = import("../../wasm/pkg");
            await this.loadWASMfuncs();

            // set the external display parameters (loaded from URL)
            this.setDisplayParams();

            // setup the SVG overlay (using D3)
            // eslint-disable-next-line no-unused-vars
            let imgDisplayWidth = document.querySelector('.image-container').clientWidth;
            let imgDisplayHeight = document.querySelector('.image-container').clientHeight;

            this.displayFac = this.rasterSize / imgDisplayWidth;
            this.overlay = d3.select(".rasterImageOverlay")
                .attr("viewBox", [-imgDisplayWidth/2, -imgDisplayHeight/2, imgDisplayWidth, imgDisplayHeight])
                .on("mousemove", (event) => { this.updateMouseCoordinate(event) })
                .call(d3.zoom()
                    .extent([[-imgDisplayWidth/2, -imgDisplayHeight/2], [imgDisplayWidth/2, imgDisplayHeight/2]])
                    .scaleExtent([1, 5])
                    .translateExtent([[-imgDisplayWidth/2, -imgDisplayHeight/2], [imgDisplayWidth/2, imgDisplayHeight/2]])
                    .on("zoom", ({transform}) => this.zoomDiffractionPlot(transform)));
            this.overlay.node();  // update the SVG
            this.$refs.LUTmanager.buildFilterString(); // intialise the fitlers

            console.log('[ Fourierpanel mounted ]');
        }
    }
</script>

<style> /* unscoped styles*/
    /* for the SVG overlay (maxima)*/
    div.maxima-tooltip {
        position: absolute;
        text-align: center;
        padding: .2rem;
        background: #313639;
        color: #f9f9f9;
        border: 0px;
        border-radius: 8px;
        pointer-events: none;
        font-size: .7rem;
        z-index: 3;
        font-family: monospace;
    }
</style>

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
        padding-top: 1rem;
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
        width: 100%;
        position: absolute;
    }

    .rasterImageOverlay{
        max-width: 100%;
        max-height: 100%;

        position: relative;
        width: inherit;
        cursor: crosshair;
        display: block;  /*THIS IS IMPORTANT!*/
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
        color: var(--bw-button-color)
    }

    .upload-window{
        width: 44rem;
        background-color: var(--color-bg);
        padding: 0.4rem 1rem 1rem 1rem;

        display: grid;
        grid-template-rows: auto 1fr auto;
        grid-template-columns: 1fr auto;
    }

    .upload-input{
        align-self: start;
        grid-row: 2;
        grid-column: 1;
    }

    .upload-preview{
        max-width: 100%;
        max-height: 40vh;
        display: inline-block;
        padding-left: 1rem;
        grid-row: 2;
        grid-column: 2;
    }

    .upload-close{
        grid-row: 3;
        grid-column: 1 / span 2;
        justify-self: end;
        margin-top: 0.7rem;
    }

    .upload-menu-title{
        grid-row: 1;
        grid-column: 1;
        display: flex;
        justify-content: space-between;
        align-items: center;
        color: var(--bw-text-color);
    }

    .upload-menu-header{
        color: var(--primary);
        font-weight: bold;
        margin-top: 0.2rem;
    }

    .upload-close-mobile{ /*hide this button on desktop*/
        display: none;
    }

    .compact-radio{
        margin: 0.1rem;
    }

    .side-by-side{
        display: flex;
    }

    .hide-button{
        margin-left: 0.5rem;
    }

    .side-by-side-slider{
        display: flex;
    }

    .current_LUT_image{
        cursor: pointer;
        z-index: 1;
    }

    .fade-enter-active {
        transition: opacity 1.5s;
    }
    .fade-enter /* .fade-leave-active below version 2.1.8 */ {
        opacity: 0;
    }

    .highlight{
        position: relative;
    }

    .highlight:after {
        display: inline-block;
        content: " ";
        height: 4px;
        left: 0;
        position: absolute;
        top: 93%;
        margin-left: 5%;
        width: calc(90%);
        z-index: -1;
    }

    .primarycol:after{
        background-color: var(--primary);
    }

    .secondarycol:after{
        background-color: var(--secondary);
    }

    .lut_controls_analytic, .lut_controls_upload{
        position: relative;
        z-index: 1;
    }

    @media only screen and (max-width: 600px) {
        .card-fft{
            height:120vw;
            padding: 0;
        }

        .display-controls-header{
            flex-direction: column;
        }

        .display-controls-drawer{
            justify-content: space-between;
            margin-top: 10px;
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
            grid-template-rows: auto auto 1fr;
            grid-template-columns: 1fr;
        }

        .side-by-side-slider{
            flex-direction: column;
        }

        .upload-preview{
            grid-row: 2;
            grid-column: 1;
            margin-bottom: 1rem;
            padding: 0;
            justify-self: center;
        }

        .upload-input{
            grid-row: 3;
            grid-column: 1;
        }

        .upload-close{ /*hide this button on mobile*/
            display: none;
        }

        .upload-close-mobile{ /*show this button on mobile*/
            display: block;
        }

        .hide-button{ margin-left: 0 }
    }
</style>

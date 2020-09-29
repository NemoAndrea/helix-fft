<template>
    <span>
    <v-menu left bottom :close-on-click=true transition="slide-y-transition" :offset-y=true >
        <template v-slot:activator="{ on: menu, attrs }">
            <v-tooltip bottom>
                <template v-slot:activator="{ on: tooltip }">
                    <v-btn v-bind="attrs" icon v-on="{ ...tooltip, ...menu }" class="display-controls-button">
                        <v-icon > mdi-palette </v-icon>
                    </v-btn>
                </template>
                <span> Select LUT </span>
            </v-tooltip>
        </template>
        <v-list shaped>
            <v-subheader>CHOOSE LUT</v-subheader>
            <v-list-item>
                <v-radio-group v-if="current_image===ImageType.ANALYTIC"  style="margin-top: 0" v-model="LUT_analytic">
                        <v-switch
                                v-model="invert_LUT_ana"
                                label="invert LUT"
                                :color="current_image"
                                style="margin-top: 0"
                        ></v-switch>
                        <v-radio :color="current_image" v-for="lut in LUT_list" :key="lut" :label="lut" :value="lut" />
                </v-radio-group>
                <v-radio-group v-if="current_image===ImageType.EXPERIMENTAL"  style="margin-top: 0" v-model="LUT_upload">
                        <v-switch
                                v-model="invert_LUT_upl"
                                label="invert LUT"
                                :color="current_image"
                                style="margin-top: 0"
                        ></v-switch>
                        <v-radio :color="current_image" v-for="lut in LUT_list" :key="lut" :label="lut" :value="lut" />
                </v-radio-group>
            </v-list-item>
        </v-list>
    </v-menu>


    <svg style="display: none">

    <!--  LUTS -->
        <filter id="grayscale">
        <!-- Grab the SourceGraphic (implicit) and convert it to grayscale -->
            <feColorMatrix type="matrix" values=".33 .33 .33 0 0
                  .33 .33 .33 0 0
                  .33 .33 .33 0 0
                  0 0 0 1 0">
            </feColorMatrix>
        </filter>
        <filter id="inferno">
            <feComponentTransfer color-interpolation-filters="sRGB">
                <feFuncR type="table" tableValues=".0.002810891 0.188894276 0.411745277 0.621527609 0.816894047 0.938706536 0.949412109  0.959399507"></feFuncR>
                <feFuncG type="table" tableValues="0.000239715 0.025561629 0.059771453 0.15592399 0.281058794 0.492124283 0.760847757 1"></feFuncG>
                <feFuncB type="table" tableValues="0.013984976 0.330141008 0.435174425 0.390325042 0.261238427 0.081251836 0.136596414 0.640626478 "></feFuncB>
            </feComponentTransfer>
        </filter>

        <filter id="mako">
            <feComponentTransfer color-interpolation-filters="sRGB">
                <feFuncR type="table" tableValues="0.04503935 0.18195582 0.25307401 0.21607792 0.20344718 0.25187832 0.54578602 0.87218969"></feFuncR>
                <feFuncG type="table" tableValues="0.01482344 0.11955283 0.23772973 0.39736958 0.56074869 0.71827158 0.8544913 0.95960708"></feFuncG>
                <feFuncB type="table" tableValues="0.02092227 0.23136943 0.48316271 0.61948028 0.65649508 0.67872193 0.69848331 0.89725384"></feFuncB>
            </feComponentTransfer>
        </filter>

        <filter id="magenta">
            <feComponentTransfer color-interpolation-filters="sRGB">
                <feFuncR type="table" tableValues="0 1"></feFuncR>
                <feFuncG type="table" tableValues="0 0"></feFuncG>
                <feFuncB type="table" tableValues="0 1"></feFuncB>
            </feComponentTransfer>
        </filter>

        <filter id="cyan">
            <feComponentTransfer color-interpolation-filters="sRGB">
                <feFuncR type="table" tableValues="0 0"></feFuncR>
                <feFuncG type="table" tableValues="0 1"></feFuncG>
                <feFuncB type="table" tableValues="0 1"></feFuncB>
            </feComponentTransfer>
        </filter>

        <filter id="green">
            <feComponentTransfer color-interpolation-filters="sRGB">
                <feFuncR type="table" tableValues="0 0"></feFuncR>
                <feFuncG type="table" tableValues="0 1"></feFuncG>
                <feFuncB type="table" tableValues="0 0"></feFuncB>
            </feComponentTransfer>
        </filter>

        <filter id="red">
            <feComponentTransfer color-interpolation-filters="sRGB">
                <feFuncR type="table" tableValues="0 1"></feFuncR>
                <feFuncG type="table" tableValues="0 0"></feFuncG>
                <feFuncB type="table" tableValues="0 0"></feFuncB>
            </feComponentTransfer>
        </filter>

    <!--Functions-->
        <filter id="minmax_a"> <!-- Filter for the analytic image -->
          <feComponentTransfer>
              <feFuncR type="gamma" :exponent="gamma_ana" :amplitude="1 / (max_ana - min_ana)" :offset="-1 / ( max_ana - min_ana) * (min_ana) + offset_ana"/>
              <feFuncG type="gamma" :exponent="gamma_ana" :amplitude="1 / (max_ana - min_ana)" :offset="-1 / ( max_ana - min_ana) * (min_ana) + offset_ana"/>
              <feFuncB type="gamma" :exponent="gamma_ana" :amplitude="1 / (max_ana - min_ana)" :offset="-1 / ( max_ana - min_ana) * (min_ana) + offset_ana"/>
          </feComponentTransfer>
        </filter>

        <filter id="minmax_b"> <!-- Filter for the experimental (uploaded) image -->
          <feComponentTransfer>
              <feFuncR type="gamma" :exponent="gamma_upl" :amplitude="1 / (max_upl - min_upl)" :offset="-1 / ( max_upl - min_upl) * (min_upl) + offset_upl"/>
              <feFuncG type="gamma" :exponent="gamma_upl" :amplitude="1 / (max_upl - min_upl)" :offset="-1 / ( max_upl - min_upl) * (min_upl) + offset_upl"/>
              <feFuncB type="gamma" :exponent="gamma_upl" :amplitude="1 / (max_upl - min_upl)" :offset="-1 / ( max_upl - min_upl) * (min_upl) + offset_upl"/>
          </feComponentTransfer>
        </filter>
    </svg>
    </span>
</template>

<script>
    import { ImageType } from "../utils/upload_utils";
    export default {
        name: "LutManager",
        props: {
            min_ana: {
                type: Number,
                default: 0
            },
            max_ana: {
                type: Number,
                default: 1
            },
            offset_ana: {
                type: Number,
                default: 0
            },
            gamma_ana: {
                type: Number,
                default: 1
            },
            min_upl: {
                type: Number,
                default: 0
            },
            max_upl: {
                type: Number,
                default: 1
            },
            offset_upl: {
                type: Number,
                default: 0
            },
            gamma_upl: {
                type: Number,
                default: 1
            },
            current_image: {
                type: String,
                default: ImageType.ANALYTIC
            }
        },
        data: () => ({
            LUT_analytic: 'grayscale',  // Current lut selection for the analytic image
            LUT_upload : 'grayscale',   // Current lut selection for the experimental (uploaded) image
            LUT_list: ['grayscale', 'inferno', 'mako', 'cyan', 'magenta','red', 'green'],
            invert_LUT_ana: false,
            invert_LUT_upl: false,
            filterString: '',
            ImageType: ImageType
        }),
        methods: {
            buildFilterString(){
                let filterStringAnalytic;
                let filterStringUpload;
                // analytic filter String
                if (this.invert_LUT_ana){
                    filterStringAnalytic = `url(#minmax_a) invert(1) url(#${this.LUT_analytic})  `;
                } else {
                    filterStringAnalytic = `url(#minmax_a) url(#${this.LUT_analytic})`;
                }
                // Experimental (upload) filter string
                if (this.invert_LUT_upl) {
                    filterStringUpload =  `url(#minmax_b) invert(1) url(#${this.LUT_upload})`;
                } else {
                    filterStringUpload =  `url(#minmax_b) url(#${this.LUT_upload})`;
                }
                this.$emit('new_filter_string', [filterStringAnalytic, filterStringUpload]) // send string to parent components
                this.$emit('lut_update') // we need to trigger event to make sure the updated plot is zoomed and panned after update
            },
        },
        mounted() {
            console.log('[LUT manager loaded]')
            // set up watches
            for ( const property of ['LUT_analytic', 'LUT_upload', 'max_ana', 'min_ana', 'offset_ana',
                'gamma_ana', 'max_upl', 'min_upl', 'offset_upl', 'gamma_upl', 'invert_LUT_ana', 'invert_LUT_upl'] ) {
                // watch
                this.$watch(property, this.buildFilterString);
            }
        }
    }
</script>

<style scoped>

</style>

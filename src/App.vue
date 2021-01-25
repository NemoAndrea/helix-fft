<template>
  <v-app>
    <v-main >
      <div class="layout-box">
        <header ><div class="logo"></div>
          <span class="darkmode-mobile"><v-tooltip v-if="!darkMode" top>
          <template v-slot:activator="{ on }">
            <v-btn
                    icon
                    color="rgba(0,0,0,0.54)"
                    @click="setDarkMode(true)"
                    v-on="on">
              <v-icon>mdi-weather-night</v-icon>
            </v-btn>
          </template>
          <span>Dark Mode</span>
        </v-tooltip>
          <v-tooltip v-if="darkMode" top>
            <template v-slot:activator="{ on }">
              <v-btn
                      icon
                      color="#f9c72a"
                      @click="setDarkMode(false)"
                      v-on="on">
                <v-icon>mdi-white-balance-sunny</v-icon>
              </v-btn>
            </template>
            <span>Light Mode</span>
          </v-tooltip></span></header>
        <div class="layout-left">
          <div class="rotated"><a href="https://nemoandrea.github.io/" target="_blank">Built by Nemo Andrea</a></div>
          <div class="rotated"><a href="https://github.com/NemoAndrea/helix-fft/blob/master/doc/examples.md" target="_blank">Examples</a></div>
          <div class="rotated"><a href="https://github.com/NemoAndrea/helix-fft" target="_blank">Documentation</a></div>
        </div>
        <div class="layout-central">
          <div class="ui-box ui-command-panel">
            <div class="card card-accent">
              <div class="card-title title-input">
                <v-text-field
                        label="Model name"
                        v-model="modelName"
                        dark
                        dense
                        color="white"
                ></v-text-field>
              </div>

              <div class="meta-button-box">
                <v-btn outlined color="white" @click="loadExample()">Load example</v-btn>
                <div class="share-buttons">
                  <v-tooltip bottom>
                    <template v-slot:activator="{ on }">
                      <v-btn icon color="white" v-on="on" @click="exportCurrentHelixFamily()">
                        <v-icon>mdi-content-copy</v-icon>
                      </v-btn>
                    </template>
                    <span>Copy link to model</span>
                  </v-tooltip>
                  <v-menu left bottom :close-on-click=true transition="slide-y-transition" :offset-y=true >
                    <template v-slot:activator="{ on, attrs }">
                      <v-btn icon color="white" v-bind="attrs" v-on="on" @click="getShareValues">
                        <v-icon>mdi-share-variant</v-icon>
                      </v-btn>
                    </template>


                    <v-list shaped>
                      <v-subheader>SHARE TO</v-subheader>
                      <v-list-item ripple>
                        <ShareNetwork
                                network="twitter"
                                :url=modelURL
                                :title="shareText"
                                hashtags="helixiser">
                          <v-icon color="#1da1f2">mdi-twitter</v-icon>
                          <span style="color:gray!important;"> Twitter </span>
                        </ShareNetwork>
                      </v-list-item>
                      <v-list-item ripple>
                        <ShareNetwork
                                network="whatsapp"
                                :url=modelURL
                                :title="shareText">
                          <v-icon color="#25d366">mdi-whatsapp</v-icon>
                          <span style="color:gray!important;"> Whatsapp </span>
                        </ShareNetwork>
                      </v-list-item >
                      <v-list-item ripple>
                        <ShareNetwork
                                network="telegram"
                                :url=modelURL
                                :title="shareText">
                          <v-icon color="#0088cc">mdi-telegram</v-icon>
                          <span style="color:gray!important;"> Telegram </span>
                        </ShareNetwork>
                      </v-list-item>
                      <v-list-item ripple>
                        <ShareNetwork
                                network="reddit"
                                :url=modelURL
                                :title="shareText">
                          <v-icon color="#ff4500">mdi-reddit</v-icon>
                          <span style="color:gray!important;"> Reddit </span>
                        </ShareNetwork>
                      </v-list-item>
                    </v-list>
                  </v-menu>
                  <v-menu max-width="var(--more-options-menu-width)" bottom :close-on-click=true transition="slide-y-transition" :offset-y=true >
                    <template v-slot:activator="{ on:menu , attrs }"><v-tooltip bottom >
                        <template v-slot:activator="{ on:tooltip }">
                          <v-btn dark icon v-bind="attrs" v-on="{ ...tooltip, ...menu }">
                            <v-icon>mdi-dots-vertical</v-icon>
                          </v-btn>
                        </template>
                        <span>More settings</span>
                    </v-tooltip></template>
                    <div class="more-options">
                    <v-card class="more-options-card">
                      <v-card-title>
                        Set distance units
                      </v-card-title>
                      <v-card-subtitle>Select a distance unit. Your choice will be included in the exported model.</v-card-subtitle>
                      <v-card-actions>
                        <v-btn v-for="unit_type in Object.keys(unitTypes)" :key="unit_type" text color="var(--primary)"
                        @click="setUnit(unit_type)">
                          {{unit_type}}
                        </v-btn>
                      </v-card-actions>
                    </v-card>
                    <v-card>
                      <v-card-title>
                        Set image dimensions
                      </v-card-title>
                      <v-card-subtitle> Set the dimensions of the diffraction pattern. The resulting image will always
                      be square. Values above the default of 512px will increase computational time.</v-card-subtitle>
                      <v-card-actions>
                        <div class="rastersize-box">
                          <v-btn icon color="var(--primary)" @click="setRasterSize(rasterSize/2)"><v-icon >mdi-minus</v-icon></v-btn>
                          {{rasterSize}}
                          <v-btn icon color="var(--primary)" @click="setRasterSize(rasterSize*2)"><v-icon>mdi-plus</v-icon></v-btn>
                        </div>
                      </v-card-actions>
                    </v-card></div>
                  </v-menu>
                </div>
              </div>
            </div>
            <div class="card">
              <parameter-panel
                      :unit="unit"
                      v-on:updateHelixFamily="setHelixFamily($event)"
                      v-on:computeHelixFamily="computeHelixFamily()"
                      v-on:updateName="modelName = arguments[0]"
                      v-on:newDisplayParams="displaySettings = arguments[0]"
                      v-on:newModelURL="modelURL = arguments[0]"
                      ref="paramPanel" />
            </div>
          </div>
          <div class="ui-box ui-realspace-panel"><div class="card card-webgl">
            <helix-display :helixFamily="helixFamily" />
          </div></div>
          <div class="ui-box ui-fft-panel">
              <fourier-panel
                      :helixFamily="helixFamily"
                      :updateCounter="updateCounter"
                      :name="modelName"
                      :unit="unit"
                      :externalDisplayParams="displaySettings"
                      :rasterSize="rasterSize"
                      v-on:exportDisplayParams="displaySettings = arguments[0]"
                      ref="fourierPanel"
                />
          </div>
        </div>
        <div class="layout-right"></div>
        <footer >
          <v-tooltip v-if="!darkMode" top>
            <template v-slot:activator="{ on }">
              <v-btn
                      icon
                      color="rgba(0,0,0,0.54)"
                      @click="setDarkMode(true)"
                      v-on="on">
                <v-icon>mdi-weather-night</v-icon>
              </v-btn>
            </template>
            <span>Dark Mode</span>
          </v-tooltip>
          <v-tooltip v-if="darkMode" top>
            <template v-slot:activator="{ on }">
              <v-btn
                      icon
                      color="#f9c72a"
                      @click="setDarkMode(false)"
                      v-on="on">
                <v-icon>mdi-white-balance-sunny</v-icon>
              </v-btn>
            </template>
            <span>Light Mode</span>
          </v-tooltip>
          <a href="https://twitter.com/cursed_tubule" target="_blank" style="margin-left: 1rem">Twitter </a>
        </footer>
      </div>
      <v-snackbar v-model="snackbar">
        {{ snackText }}
        <template v-slot:action="{ attrs }">
          <v-btn
                  color="pink"
                  text
                  v-bind="attrs"
                  @click="snackbar = false">
            Close   </v-btn>        </template>
      </v-snackbar>
    </v-main>
  </v-app>
</template>

<script>
import HelixDisplay from "./components/HelixDisplay";
import FourierPanel from "./components/FourierPanel";
import ParameterPanel from "./components/ParameterPanel";
import Vue from 'vue'
import VueSocialSharing from 'vue-social-sharing'
import { Unit } from './utils/app_utils'

Vue.use(VueSocialSharing);

export default {
  name: 'App'
  ,
  components:{
    ParameterPanel,
    HelixDisplay,
    FourierPanel
  },
  data: () => ({
    helixFamily: [],
    displaySettings: {},
    modelName: '',
    updateCounter: 0,
    modelURL: '',
    shareText: '',
    darkMode: false,
    rasterSize: 512, // we default to 512 as a rastersize for the canvas. Setting to higher values is possible.
    unit: Unit.angstrom,  // we default to angstrom as units
    unitTypes: Unit, // so the rendered template can access types
    snackbar: false,
    snackText: 'unspecified',
  }),
  watch: {
    modelName: {
      handler: function () { this.updatePageTitle() }
    },
  },
  methods: {
    loadExample() {  // load an example via search query string. Will reload the page.
      window.location.search = 'name=B-DNA&m=1&radius=1&rise=0.34&frequency=10&unit_size=0.18' +
              '&radius=1&rise=0.34&frequency=10&unit_size=0.18&rotation=143';
      },

    setHelixFamily (newHelixFamily) {
      this.helixFamily = newHelixFamily;
    },

    computeHelixFamily () {
      this.updateCounter ++;  // log how many times we have computed FFT. Triggers calculation in FourierPanel.
      console.log(`>> We have computed ${this.updateCounter} FFTs `)
    },

    exportCurrentHelixFamily (){
      this.$refs.fourierPanel.exportDisplayParams();  // we must first fetch the displaySettings (from fourierpanel)

      // directly call method 'exportmode()' in ParameterPanel child (required ref to be defined in html above)
      this.$refs.paramPanel.exportModel(this.modelName, this.displaySettings)
    },

    getShareValues(){
      // first we update the share link
      this.$refs.fourierPanel.exportDisplayParams();  // we must first fetch the displaySettings (from fourierpanel)
      this.$refs.paramPanel.emitModelURL(this.modelName, this.displaySettings)
      // now this.modelURL  string should be updated

      // then we update the sharetext
      if (this.modelName) {
        this.shareText = 'Check out diffraction pattern of (helical) ' + this.modelName + '. ';
      } else {
        this.shareText = 'Check out the diffraction pattern of this helical structure. ';
      }
    },

    updatePageTitle(){
      if (this.modelName) {
        document.title = this.modelName + ' | Helixiser';
      } else {
        document.title = 'Helixiser';
      }
    },

    setDarkMode( shouldBeOn ) {
      if (shouldBeOn) {
        this.darkMode = true;
        document.body.setAttribute('data-theme', 'dark');
        this.$vuetify.theme.dark = this.darkMode;
        localStorage.setItem("dark_theme", this.darkMode.toString());
      } else {
        this.darkMode = false;
        document.body.removeAttribute('data-theme');
        this.$vuetify.theme.dark = this.darkMode;
        localStorage.setItem("dark_theme", this.darkMode.toString());

      }
    },

    setUnit( new_unit ) {
      if ( Object.keys(Unit).includes(new_unit) ) {  // is the unit in localstorage valid (i.e. defined in the Unit enum)
        this.unit = Unit[new_unit];
        localStorage.setItem("distance_unit", new_unit)
      } else {  // something went wrong. This should never be the case unless you manually edit the local storage.
        this.snackText = `Failed to set distance unit to "${new_unit}". Distance set to "nm" instead.`;
        this.snackbar = true;
        localStorage.setItem("distance_unit", 'angstrom')
      }
    },

    setRasterSize( size ) {
      if ( Math.log2(size) % 1 === 0 ) {  // is the rastersize a power of 2?
        localStorage.setItem("canvas_rastersize", String(size));
        this.rasterSize = size;
        console.log('New rastersize set: ', size)
      } else {
        this.snackText = `Failed to set canvas size to ${size}. Distance set to 512 instead.`;
        this.snackbar = true;
        localStorage.setItem("canvas_rastersize", String(512));
        this.rasterSize = 512;
      }
    }
  },
  mounted() {
    console.log('[ Loaded main component ]');
    //load last dark theme setting from local storage
    const last_theme_setting = localStorage.getItem("dark_theme");
    last_theme_setting === 'true' ? this.setDarkMode( true ) : this.setDarkMode( false );
    // load rastersize from local storage. If it wasn't set before, default to 512.
    const canvas_rastersize = localStorage.getItem("canvas_rastersize");
    canvas_rastersize !== null ? this.setRasterSize( Number(canvas_rastersize) ) : localStorage
            .setItem("canvas_rastersize", String(this.rasterSize));
    // load unit choice from local storage.
    const local_unit = localStorage.getItem("distance_unit");
    local_unit !== null ? this.setUnit( local_unit ) : localStorage.setItem("distance_unit", 'angstrom');

    // check if we are running helixiser locally using Node.js
    if(typeof process === 'object') {
      console.log('[Node] Running from Node')

    }
  }
};
</script>

<style>
  :root {
    --primary: salmon; /*#FA8072*/
    --selection: pink;
    --mygrey: #ececec;
    --secondary: #808080;
    /*theme specific variables*/
    --color-bg: #ffffff;
    --bw-text-color: rgba(0,0,0,0.95);
    --bw-faint-color: rgba(0,0,0,0.30);
    --bw-button-color: rgba(0,0,0,0.54);
    --bw-slider-track-color: darkgrey;
    /*tool variables*/
    --more-options-menu-width: 40%;
  }

  ::selection {
    background-color: var(--selection);
  }

  [data-theme="dark"] {
    --color-bg: #121212;
    --bw-text-color: rgba(255,255,255,0.87);
    --bw-faint-color: rgba(255,255,255,0.30);
    --primary: cadetblue; /*#5f9ea0*/
    --bw-button-color: rgba(255,255,255,0.60);
    --bw-slider-track-color: rgba(255,255,255,0.10);
    --selection: #caeaeb;
  }


  .layout-box{
    display: grid;
    grid-template: auto 1fr auto / auto 1fr auto;
    width: 100vw;
    height: 100vh;
    overflow-y: scroll;
    background-color: var(--color-bg);
  }

  header{
    grid-column: 1 / 4;
    margin: 2rem 4rem 1rem 4rem;
    display: flex;
    justify-content: space-between;
  }

  .layout-left{
    grid-column: 1 / 2;
    color: var(--primary);
    margin: 1rem;
    font-size: 1.1rem;
    font-weight: bold;
  }

  .layout-central{
    grid-column: 2 / 3;
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
  }

  .layout-right{
    grid-column: 3 / 4;
    background-color: green;
  }

  footer{
    grid-column: 1 / 4;
    color: var(--primary);
    text-align: right;
    margin: 0.2rem 0.5rem;
    font-weight: bold;
  }

  .logo{
    height: 3rem;
    width: 14rem;
    background-color: var(--primary);
    mask: url("../public/logo_text.svg") no-repeat center / contain;;
  }

  .ui-box{
    margin: 0.5rem;
    min-height: 80vh;
  }

  .ui-command-panel{
    display: grid;
    grid-template-rows: auto 1fr;
    flex: 0 1 26rem;
  }

  .ui-realspace-panel{
    flex: 0 1 15rem;
    background-color: var(--color-bg);
    z-index: 2;
  }

  .ui-fft-panel{
    flex: 0 1 38rem;
  }

  .rotated{
    writing-mode: vertical-rl;
    transform: rotate(180deg);
    margin-bottom: 3rem;
  }

  .card{
    width: 100%;
    /*background-color: var(--mygrey);*/
    padding: 2rem;
    border:2px solid var(--primary);
    border-radius: 0 30px 30px 30px;
    color: var(--bw-text-color);
  }

  .card-title{
    font-size: 1.3rem;
    margin-bottom: 0.5rem;
  }

  .card-title-float{
    margin: 1rem;
    padding: 0.6rem;
    position: absolute;
    font-weight: bold;
    z-index: 2;
  }

  .card-accent{
    background-color: var(--primary);
    margin-bottom: 1rem;
    color: white;
  }

  .card-webgl{
    border-radius: 9px;
    height: 100%;
    padding: 0;
    position: relative;
    overflow: hidden;
  }

  .card-display{
    margin-bottom: 1rem;
  }

  .more-options-card {
    margin-bottom: 8px;
  }


  .meta-button-box{
    display: flex;
    justify-content: space-between;
  }

  .darkmode-mobile {
    display: none;
  }

  .rastersize-box {
    display: flex;
    align-items: center;
  }

  .rastersize-icon {
    color: var(--primary);
  }

  a {text-decoration: none; color: var(--primary) !important;}

  @media only screen and (max-width: 600px) {
    :root{
      --more-options-menu-width: 92%;
    }


    .logo{
      height: 2rem;
      width: 9.3rem;
      fill: green;
    }

    header{
      margin: 1rem 1rem 0 1rem;
    }

    footer{
      grid-row: 4;
      grid-column: 1;
    }

    .layout-box{  /*we change the grid layout*/
      grid-template:  1fr  / auto auto 1fr auto ;
    }

    .layout-left {
      grid-row: 2;
      grid-column: 1;

      display: flex;
      margin: 0;
      padding: 0.1rem 0.7rem;
      font-size: 15px;
      justify-content: space-between;
      width: 100vw;
    }

    .rotated{
      writing-mode: horizontal-tb;
      transform: rotate(0);
      margin: 0;
      padding: 0;
      width: 100%;
    }

    .layout-right{
      display: none;
    }

    .card{
      padding: 1.5rem;
    }

    .card-webgl{
      padding: 0;
    }


    .layout-central{
      margin: 0.2rem;
      grid-column: 1;
      grid-row: 3;
    }

    .card-title{
      font-size: 1.1rem;
      font-weight: bold;
    }



    .ui-realspace-panel, .ui-fft-panel, .ui-command-panel {
      flex-basis: 100%;
    }

    .darkmode-mobile {
      display: block;
    }
  }
</style>

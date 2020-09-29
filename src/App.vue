<template>
  <v-app>
    <v-main >
      <div class="layout-box">
        <header ><div class="logo"></div>
          <span class="darkmode-mobile"><v-tooltip v-if="!darkMode" top>
          <template v-slot:activator="{ on }">
            <v-btn
                    icon
                    color="#121212"
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
          <div class="rotated"><a href="https://github.com/NemoAndrea/helix-fft" target="_blank">Source & Documentation</a></div>
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

                </div>
              </div>
            </div>
            <div class="card">
              <parameter-panel
                      v-on:updateHelixFamily="setHelixFamily($event)"
                      v-on:computeHelixFamily="computeHelixFamily()"
                      v-on:updateName="modelName = arguments[0]"
                      v-on:newDisplayParams="displaySettings = arguments[0]"
                      v-on:newModelURL="modelURL = arguments[0]"
                      ref="paramPanel" />
            </div>
          </div>
          <div class="ui-box ui-realspace-panel"><div class="card card-webgl">
            <div class="card-title-float card-title">Realspace helix</div>
            <helix-display :helixFamily="helixFamily" />
          </div></div>
          <div class="ui-box ui-fft-panel">
              <fourier-panel
                      :helixFamily="helixFamily"
                      :updateCounter="updateCounter"
                      :name="modelName"
                      :externalDisplayParams="displaySettings"
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
                      color="#121212"
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
    </v-main>
  </v-app>
</template>

<script>
import HelixDisplay from "./components/HelixDisplay";
import FourierPanel from "./components/FourierPanel";
import ParameterPanel from "./components/ParameterPanel";
import Vue from 'vue'
import VueSocialSharing from 'vue-social-sharing'

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
  }),
  watch: {
    modelName: {
      handler: function () { this.updatePageTitle() }
    },
  },
  methods: {
    loadExample() {
      window.location.hash += '#name=B-DNA&m=1###&radius=1&rise=0.34&frequency=10&unit_size=0.18' +
              '&radius=1&rise=0.34&frequency=10&unit_size=0.18&rotation=143';
      location.reload()
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
    }
  },
  mounted() {
    console.log('[ Loaded main component ]');
    //load last dark theme setting from local storage
    const last_theme_setting = localStorage.getItem("dark_theme");
    last_theme_setting === 'true' ? this.setDarkMode( true ) : this.setDarkMode( false );
  }
};
</script>

<style>
  :root {
    --primary: salmon; /*#FA8072*/
    --mygrey: #ececec;
    --secondary: #808080;
    /*theme specific variables*/
    --color-bg: #ffffff;
    --bw-text-color: rgba(0,0,0,0.95);
    --bw-button-color: rgba(0,0,0,0.54);
    --bw-slider-track-color: darkgrey;
  }

  ::selection {
    background-color: pink;
  }

  [data-theme="dark"] {
    --color-bg: #121212;
    --bw-text-color: rgba(255,255,255,0.87);
    --primary: cadetblue;
    --bw-button-color: rgba(255,255,255,0.60);
    --bw-slider-track-color: rgba(255,255,255,0.10);
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

  .meta-button-box{
    display: flex;
    justify-content: space-between;
  }

  .darkmode-mobile {
    display: none;
  }

  a {text-decoration: none; color: var(--primary) !important;}

  @media only screen and (max-width: 600px) {

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

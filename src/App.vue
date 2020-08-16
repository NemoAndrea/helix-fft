<template>
  <v-app>
    <v-main>
      <div class="layout-box">
        <header >Helical FFT</header>
        <div class="layout-left"><div class="rotated">Built by Nemo Andrea</div></div>
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
              <v-tooltip bottom>
                <template v-slot:activator="{ on }">
                  <v-btn icon color="white" v-on="on" @click="exportCurrentHelixFamily()">
                    <v-icon>mdi-export-variant</v-icon>
                  </v-btn>
                </template>
                <span>Export model to shareable URL in clipboard</span>
              </v-tooltip>
            </div>
            </div>
            <div class="card">
              <parameter-panel
                      v-on:updateHelixFamily="setHelixFamily($event)"
                      v-on:computeHelixFamily="computeHelixFamily()"
                      v-on:updateName="modelName = arguments[0]"
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
                />
          </div>
        </div>
        <div class="layout-right"></div>
        <footer ><a href="https://twitter.com/cursed_tubule" target="_blank">Twitter </a></footer>
      </div>
    </v-main>
  </v-app>
</template>

<script>
import HelixDisplay from "./components/HelixDisplay";
import FourierPanel from "./components/FourierPanel";
import ParameterPanel from "./components/ParameterPanel";

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
    modelName: '',
    updateCounter: 0
  }),
  methods: {
    loadExample() {
      window.location.hash += '#name=B-DNA&radius=1&rise=0.34&frequency=10&unit_size=0.18' +
              '&radius=1&rise=0.34&frequency=10&unit_size=0.18&offset=1.35';
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
      // directly call method 'exportmode()' in ParameterPanel child (required ref to be defined in html above)
      this.$refs.paramPanel.exportModel(this.modelName)
    },
  },
  mounted() {
    console.log('[ Loaded main component ]');
  }
};
</script>

<style>
  :root {
    --primary: salmon;
    --mygrey: #ececec;
  }

  ::selection {
    background-color: pink;
  }

  .layout-box{
    display: grid;
    grid-template: auto 1fr auto / auto 1fr auto;
    width: 100vw;
    height: 100vh;
    overflow-y: scroll;
  }

  header{
    grid-column: 1 / 4;
    font-size: 2rem;
    margin: 2rem 0 1rem 4rem;
    color: var(--primary);
    font-weight: bold;
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
    margin: 0.5rem;
    font-weight: bold;
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
  }

  .ui-fft-panel{
    flex: 0 1 38rem;
  }

  .rotated{
    writing-mode: vertical-rl;
    transform: rotate(180deg);
  }

  .card{
    width: 100%;
    /*background-color: var(--mygrey);*/
    padding: 2rem;
    border:2px solid var(--primary);
    border-radius: 0 30px 30px 30px;
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

  a {text-decoration: none; color: var(--primary) !important;}
</style>

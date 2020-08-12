<template>
  <v-app>
    <v-main>
      <div class="layout-box">
        <header >Helical FFT</header>
        <div class="layout-left"><div class="rotated">Built by Nemo Andrea</div></div>
        <div class="layout-central">
          <div class="ui-box ui-command-panel">
            <div v-ripple class="card card-accent elevation-3">
              <div class="card-title">Metadata card</div>
              <v-btn outlined color="white" @click="loadExample()">Load example</v-btn>
              <br>current helix-family parameters: {{helixFamily}}
            </div>
            <div v-ripple class="card elevation-3">
              <div class="card-title">Helix parameters</div>
              <parameter-panel v-on:updateHelixFamily="helixFamily = arguments[0]"/>
            </div>
          </div>
          <div class="ui-box ui-realspace-panel"><div class="card card-webgl elevation-3">
            <div class="card-title-float card-title">Realspace helix</div>
            <helix-display :helixFamily="helixFamily" />
          </div></div>
          <div class="ui-box ui-fft-panel">
            <div v-ripple class="card card-display elevation-3">Display Controls</div>
            <div  v-ripple class="card card-fft elevation-3">FFT of helix (theory and computed)
              <fourier-panel :helixFamily="helixFamily" />
            </div>
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
  }),
  methods: {
    loadExample() {
      this.helixFamily = [ {'radius': 10, 'rise': 0.9, 'frequency': 13, 'unit_size': 2} ]
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
    min-height: 50vh;
  }

  .ui-command-panel{
    display: grid;
    grid-template-rows: auto 1fr;
    flex: 0 1 24rem;
  }

  .ui-realspace-panel{
    flex: 0 1 15rem;
  }

  .ui-fft-panel{
    display: grid;
    grid-template-rows: 1fr auto;
    flex: 0 1 40rem;
  }

  .rotated{
    writing-mode: vertical-rl;
    transform: rotate(180deg);
  }

  .card{
    width: 100%;
    background-color: var(--mygrey);
    padding: 2rem;

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

  .card-fft{
    border-radius: 0;
    /*aspect-ratio: 1 / 1;*/
  }

  a {text-decoration: none; color: var(--primary) !important;}
</style>

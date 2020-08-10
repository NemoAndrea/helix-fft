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
            </div>
            <div v-ripple class="card elevation-3">
              <div class="card-title">Helix parameters</div>
              <v-form
                      ref="form"
                      v-model="valid">

                <v-text-field
                        v-model="radius_form"
                        :rules="numberRules"
                        label="Radius"
                        required
                        filled
                        color="var(--primary)"
                        suffix="nm"
                        type="number"
                ></v-text-field>

                <v-text-field
                        v-model="rise_form"
                        :rules="numberRules"
                        label="Rise"
                        required
                        filled
                        color="var(--primary)"
                        suffix="nm"
                        type="number"
                ></v-text-field>

                <v-text-field
                        v-model="frequency_form"
                        :rules="numberRules"
                        label="Subunits per pitch"
                        required
                        filled
                        color="var(--primary)"
                        suffix="nm"
                        type="number"
                ></v-text-field>

                <v-text-field
                        v-model="unit_size_form"
                        :rules="numberRules"
                        label="Unit size"
                        required
                        filled
                        color="var(--primary)"
                        suffix="nm"
                        type="number"
                ></v-text-field>

                <v-btn
                        :disabled="!valid"
                        color="var(--primary)"
                        @click="computeHelix">
                  Compute
                </v-btn>

              </v-form>
            </div>
          </div>
          <div class="ui-box ui-realspace-panel"><div class="card card-webgl elevation-3">
            <div class="card-title-float card-title">Realspace helix</div>
            <helix-display :radius="radius" :rise="rise" :frequency="frequency" :unit_size="unit_size" />
          </div></div>
          <div class="ui-box ui-fft-panel">
            <div v-ripple class="card card-display elevation-3">Display Controls</div>
            <div  v-ripple class="card card-fft elevation-3">FFT of helix (theory and computed)
              <fourier-panel :radius="radius" :rise="rise" :frequency="frequency" :unit_size="unit_size" />
            </div>
          </div>
        </div>
        <div class="layout-right"></div>
        <footer >Twitter</footer>
      </div>
    </v-main>
  </v-app>
</template>

<script>
import HelixDisplay from "./components/HelixDisplay";
import FourierPanel from "./components/FourierPanel";

export default {
  name: 'App'
  ,
  components:{
    HelixDisplay,
    FourierPanel
  },
  data: () => ({
    valid: false,
    radius_form: "",
    radius: -1,
    rise_form: "",
    rise: -1,
    frequency_form: "",
    frequency: -1,
    unit_size_form: "",
    unit_size: -1,
    numberRules: [v => !!v || 'parameter is required',],
  }),
  methods: {
    computeHelix() {
      this.radius = Number(this.radius_form);
      this.rise = Number(this.rise_form);
      this.frequency = Number(this.frequency_form);
      this.unit_size = Number(this.unit_size_form);
    },
    loadExample() {
      this.radius = this.radius_form = 10;
      this.rise = this.rise_form = 0.9;
      this.frequency = this.frequency_form = 13;
      this.unit_size = this. unit_size_form = 2;
    }
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
  }

  .card-accent{
    background-color: var(--primary);
    margin-bottom: 1rem;
    color: white;
  }

  .card-webgl{
    border-radius: 7px 7px 7px 7px;
    height: 100%;
    padding: 0;
    position: relative;
  }

  .card-display{
    margin-bottom: 1rem;
  }

  .card-fft{
    border-radius: 0;
    /*aspect-ratio: 1 / 1;*/
  }


</style>

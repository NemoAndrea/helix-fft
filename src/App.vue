<template>
  <v-app>
    <v-main>
      <div class="layout-box">
        <header >Helical FFT</header>
        <div class="layout-left"><div class="rotated">Built by Nemo Andrea</div></div>
        <div class="layout-central">
          <div class="ui-box ui-command-panel">
            <div class="card card-accent">
              <div class="card-title">Metadata card</div>
              <v-btn disabled>Load example</v-btn>
            </div>
            <div class="card">
              <div class="card-title">Helix parameters</div>
              <v-form
                      ref="form"
                      v-model="valid">

                <v-text-field
                        v-model="radius"
                        :rules="numberRules"
                        label="Radius"
                        required
                        filled
                        color="var(--primary)"
                        suffix="nm"
                        type="number"
                ></v-text-field>

                <v-text-field
                        v-model="rise"
                        :rules="numberRules"
                        label="Rise"
                        required
                        filled
                        color="var(--primary)"
                        suffix="nm"
                        type="number"
                ></v-text-field>

                <v-text-field
                        v-model="frequency"
                        :rules="numberRules"
                        label="Frequency"
                        required
                        filled
                        color="var(--primary)"
                        suffix="nm"
                        type="number"
                ></v-text-field>

                <v-text-field
                        v-model="unit_size"
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
          <div class="ui-box ui-realspace-panel"><div class="card card-webgl">
            <div class="card-title-float card-title">Realspace helix</div>
          </div></div>
          <div class="ui-box ui-fft-panel">
            <div class="card card-display">Display Controls</div>
            <div class="card card-fft">FFT of helix (theory and computed)</div>
          </div>
        </div>
        <div class="layout-right"></div>
        <footer >Twitter</footer>
      </div>
    </v-main>
  </v-app>
</template>

<script>
import * as THREE from '../ext/three.module.js';

export default {
  name: 'App'
  ,

  data: () => ({
    valid: false,
    radius: "",
    rise: "",
    frequency: "",
    unit_size: "",
    numberRules: [v => !!v || 'parameter is required',],
  }),
  methods: {
    computeHelix() {
      console.log('Helix parameters: ' + this.radius + this.rise + this.frequency + this.unit_size);
      const scalefac = 10/this.radius;  // the prefactor is an empirical value and might need to be made more robust

      // get the div we will draw in
      let container = document.querySelector('.card-webgl' );
      let containerinfo = container.getBoundingClientRect();

      // set up the camera
      THREE.Object3D.DefaultUp = new THREE.Vector3( 0,0,1);
      let camera = new THREE.PerspectiveCamera( 40, containerinfo.width/containerinfo.height, 0.1, 15000 );
      camera.position.z = 50;
      const cam_radius = 160;
      camera.position.x = cam_radius;
      let cam_angle = 0;  // initialise to 0
      camera.lookAt( new THREE.Vector3(0,0,50) );

      let scene = new THREE.Scene();

      // set up helical building blocks
      let geometry = new THREE.SphereGeometry( this.unit_size*scalefac, 12,12);
      let material = new THREE.MeshNormalMaterial();

      const amount = Math.ceil(100 / (this.rise*scalefac));

      // build the helix
      for ( var i = 0; i < amount ; i ++ ) {
        let object = new THREE.Mesh( geometry, material );
        object.position.x = this.radius*scalefac*Math.cos(i*2*Math.PI/this.frequency);
        object.position.y = this.radius*scalefac*Math.sin(i*2*Math.PI/this.frequency);
        object.position.z = i*this.rise*scalefac;


        scene.add( object );
      }

      // setup render window
      let renderer = new THREE.WebGLRenderer( { antialias: true, alpha: true} );
      renderer.setPixelRatio( window.devicePixelRatio );
      renderer.setSize( containerinfo.width, containerinfo.height );
      container.appendChild( renderer.domElement );

      // function that is repeatedly automatically called
      function animate() {
        requestAnimationFrame( animate );
        render();
      }
      function render(){
        camera.position.x = cam_radius*Math.cos(cam_angle);
        camera.position.y = cam_radius*Math.sin(cam_angle);
        cam_angle = cam_angle + 0.01;

        camera.lookAt( new THREE.Vector3(0,0,50) );
        renderer.render( scene, camera );

      }
      animate();
    },
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
    padding-top: 100%;
    /*aspect-ratio: 1 / 1;*/
  }


</style>

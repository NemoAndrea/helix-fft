<template>
    <div class="helix-display">
        <div v-show="helixFamily.length > 0" class="realspace-controls">
            <div>

            <v-tooltip top>
                <template v-slot:activator="{ on }">
                    <v-btn
                            dark
                            fab
                            color="var(--primary)"
                            @click="rotate = !rotate"
                            v-on="on"
                    >
                        <v-icon v-if="rotate">mdi-pause</v-icon>
                        <v-icon v-if="!rotate">mdi-play</v-icon>
                    </v-btn>
                </template>
                <span>Toggle auto-rotation</span>
            </v-tooltip>
            </div>
            <div class="button-fullscreen">
            <v-tooltip top>
                <template v-slot:activator="{ on }">
                    <v-btn
                            dark
                            fab
                            color="var(--primary)"
                            @click="toggleFullscreen"
                            v-on="on"
                    >
                        <v-icon v-if="fullscreen">mdi-fullscreen-exit</v-icon>
                        <v-icon v-if="!fullscreen">mdi-fullscreen</v-icon>
                    </v-btn>
                </template>
                <span>Fullscreen</span>
            </v-tooltip>
            </div>
        </div>
    </div>
</template>

<script>
    import * as THREE from '../../ext/three.module.js';
    import OrbitControls from '../../ext/OrbitControls'

    export default {
        name: "HelixDisplay",
        props: {
            helixFamily: {
                type: Array,
            },
        },
        watch: {
            // eslint-disable-next-line no-unused-vars
            helixFamily: {
                handler: function () {
                    this.refreshHelix()
                },
                deep: true
            },
        },
        data: () => ({
            scene: '',
            rotationRate: 0.01,
            rotate: true,
            fullscreen: false,
            camera: '',
            renderer: '',
            container: '',
            colors: [
                new THREE.Color(211/255, 94/255, 96/255),  // red
                new THREE.Color(51/200, 62/200, 91/200),   // indigo
                new THREE.Color(128/255, 133/255, 133/255),  // gray
                new THREE.Color(144/255, 103/255, 167/255),
                new THREE.Color(114/255, 147/255, 203/255),
                new THREE.Color(225/255, 151/255, 76/255),
                new THREE.Color(132/255, 186/255, 91/255),
                new THREE.Color(204/255, 194/255, 16/255),
            ]
        }),
        methods:{
            refreshHelix(){
                console.log('redrawing the helix (three.js)');

                // clear the old helices
                while(this.scene.children.length > 0){
                    this.scene.remove(this.scene.children[0]);
                }

                // compute relevant scale to draw helix at (we also consider the unit size)
                let max_radius= Number(this.helixFamily[0]['radius']) + Number(this.helixFamily[0]['unit_size']) ;
                for (let helix of this.helixFamily) {
                    let draw_radius = Number(helix['radius']) + Number(helix['unit_size']);
                    if ( 1/ draw_radius < 1/max_radius ){
                        max_radius = draw_radius
                    }
                }
                const scalefac = 10/max_radius;  // the prefactor is an empirical value and might need to be made more robust

                // define handedness
                let handedness = 1;  // right handed=1, left handed =-1

                for ( let [ind, helix] of this.helixFamily.entries()) {
                     if (ind < this.colors.length) {
                        this.material = new THREE.MeshToonMaterial( { color: this.colors[ind] });
                    }

                    // set handedness for this helix
                    if (helix['handedness'] === 'left') {
                        handedness = -1;
                    } else {
                        handedness = 1;
                    }

                    let amount = Math.ceil( 100 / ( helix['rise']*scalefac ) );  // number of helical units to draw
                    if (amount > 1000) { amount = 1000}
                    let geometry = new THREE.SphereGeometry( helix['unit_size']*scalefac, 12,12 );

                    // build the helix
                    for ( var i = 0; i < amount ; i ++ ) {
                        let object = new THREE.Mesh( geometry, this.material );
                        let angle = Number(helix['rotation'])/180*Math.PI +  handedness*i*2*Math.PI / helix['frequency'] ;
                        object.position.x = helix['radius'] * scalefac * Math.cos( angle );
                        object.position.y = helix['radius'] * scalefac * Math.sin( angle);
                        object.position.z = i*helix['rise'] * scalefac + helix['offset']*scalefac;

                        this.scene.add( object );
                    }
                }

                this.addLights();
                this.refreshDisplay( this.container );
            },

            addLights(){
                let light = new THREE.AmbientLight( 0xffffff, 0.75  ); // soft white light
                this.scene.add( light );

                const directionalLight = new THREE.DirectionalLight( 0xffffff, 1 );
                directionalLight.position.set( 1, -1, 1 ).normalize();
                directionalLight.intensity = 0.25
                directionalLight.castShadow = true;
                this.scene.add( directionalLight );
            },

            setupDisplay(element_name){
                // get the div we will draw in
                this.container = document.querySelector( element_name );
                this.scene = new THREE.Scene();
                this.scene.background = new THREE.Color( );


                // setup render window
                this.renderer = new THREE.WebGLRenderer( { antialias: true} );
                this.renderer.setPixelRatio( window.devicePixelRatio );
                this.renderer.setSize( this.container.offsetWidth , this.container.offsetHeight );
                let GLwindow = this.container.appendChild( this.renderer.domElement );
                GLwindow.style.display = 'block'; // really important to prevent ghost whitespace below canvas

                // set up the camera
                THREE.Object3D.DefaultUp = new THREE.Vector3( 0,0,1);
                this.camera = new THREE.PerspectiveCamera( 40, this.container.offsetWidth/this.container.offsetHeight,
                    0.1, 5000 );
                const cam_radius = 160;
                const cam_height = 50;

                // further camera setup
                let controls = new OrbitControls( this.camera, this.renderer.domElement );
                controls.autoRotate = this.rotate;
                controls.target.set( 0, 0, cam_height );
                this.camera.position.set( cam_radius, 0, cam_height );
                controls.enableKeys = false;
                controls.update();

                window.addEventListener( 'resize', () => this.refreshDisplay( this.container ), false );  // detect if page resize

                // function that is repeatedly automatically called
                const animate = () => {
                    requestAnimationFrame( animate );
                    render();
                };

                const render =  () => {
                    controls.update();
                    controls.autoRotate = this.rotate;
                    this.renderer.render( this.scene, this.camera );
                    this.camera.lookAt( new THREE.Vector3(0,0,cam_height) );
                };

                console.log('[ Realspace Helix display ready. ]');
                animate()
            },

            refreshDisplay(container) {
                //console.log('Refreshing aspect ratio plot');
                this.camera.aspect = container.offsetWidth / container.clientHeight ;
                this.camera.updateProjectionMatrix();
                this.renderer.setSize( container.offsetWidth, container.clientHeight  );
            },

            toggleFullscreen(){
                this.fullscreen = !this.fullscreen;

                let card = document.querySelector( '.ui-realspace-panel' );

                if (this.fullscreen) {
                    console.log('going fullscreen, baby');
                    card.style.position = 'absolute';
                    card.style.width = '90vw';
                } else {
                    console.log('leaving fullscreen');
                    card.style.position = 'static';
                    card.style.width = '0px';
                }
                this.refreshDisplay(this.container)
            }
        },
        mounted() {
            this.setupDisplay( '.helix-display' )
        }
    }
</script>

<style scoped>
    .helix-display{
        height: 100%;
        width: 100%;
        position: relative;
        z-index: 1;
    }

    .realspace-controls{
        padding: 1rem;
        position: absolute;
        display: flex;
        justify-content: space-between;
        width: 100%;
        bottom: 10px
    }

    @media only screen and (max-width: 600px) {
        .button-fullscreen{
            display: none;
        }
    }
</style>

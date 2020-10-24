<template>
    <div class="helix-display">
        <div class="top-realspace-buttons" v-if="fullscreen">
            <v-tooltip bottom  v-if="!planarMode">
                <template v-slot:activator="{ on }">
                    <v-btn dark color="var(--primary)" v-on="on" fab
                            @click="planarMode = true"
                            class="top-realspace-item">
                        <v-icon>mdi-dots-grid</v-icon>
                    </v-btn>
                </template>
                <span>Unroll into lattice</span>
            </v-tooltip>
            <v-tooltip bottom v-if="planarMode">
                <template v-slot:activator="{ on }">
                    <v-btn dark color="var(--primary)" v-on="on" fab
                           @click="planarMode = false"
                           class="top-realspace-item">
                        <v-icon>mdi-dna</v-icon>
                    </v-btn>
                </template>
                <span>Roll up lattice into helix</span>
            </v-tooltip>
        </div>
        <div v-show="helixFamily.length > 0" class="realspace-controls">
            <div>
            <v-tooltip right>
                <template v-slot:activator="{ on }">
                    <v-btn dark

                            fab
                            color="var(--primary)"
                            @click="rotate = !rotate"
                            v-on="on"
                            class="left-realspace-item"
                            :hidden="planarMode"
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
    import * as THREE from 'three';
    import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls'
    import { OutlineEffect } from 'three/examples/jsm/effects/OutlineEffect.js';

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
            planarMode: {
                handler: function () {
                    this.refreshHelix()
                },
            }
        },
        data: () => ({
            scene: '',
            rotationRate: 0.01,
            rotate: true,
            fullscreen: false,
            camera: '',
            cam_radius: 160,
            cam_height: 50,
            renderer: '',
            controls: null,
            raycaster: null,
            mouse:null,
            yaxis:null,
            xaxis: null,
            effect: '',
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
            ],
            planarMode: false // display helix as 2D lattice? (default = no = false)
        }),
        methods:{
            refreshHelix() {
                console.log('redrawing the helix (three.js)');
                this.planarMode ? this.drawLattice() : this.drawHelix()
            },

            drawHelix() {
                console.log('drawing a realspace helix...');
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
                    let geometry = new THREE.SphereGeometry( helix['unit_size']*scalefac, 32,16 );

                    // build the helix
                    for ( var i = 0; i < amount ; i ++ ) {
                        let object = new THREE.Mesh( geometry, this.material.clone() );
                        let angle = Number(helix['rotation'])/180*Math.PI +  handedness*i*2*Math.PI / helix['frequency'] ;
                        object.position.x = helix['radius'] * scalefac * Math.cos( angle );
                        object.position.y = helix['radius'] * scalefac * Math.sin( angle);
                        object.position.z = i*helix['rise'] * scalefac + helix['offset']*scalefac;

                        this.scene.add( object );
                    }
                }

                this.controls.enableRotate = true; // make sure you can rotate with the mouse.
                this.addLights();
                this.refreshDisplay( this.container );
            },

            drawLattice() {
                console.log('unfolding helix into lattice and drawing lattice...');
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

                const draw_circumference = Number(this.helixFamily[0]['radius']) * scalefac * 2 * Math.PI; // needed for display only

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
                    let geometry = new THREE.SphereGeometry( helix['unit_size']*scalefac, 32,16 );

                    const circumference = helix['radius'] * scalefac * 2*Math.PI;

                    // build the lattice that would be obtained by unfolding the helix
                    for ( var i = 0; i < amount ; i ++ ) {
                        let object = new THREE.Mesh( geometry, this.material.clone() );
                        let angle = Number(helix['rotation'])/180*Math.PI +  handedness*i*2*Math.PI / helix['frequency'] ;
                        // we now take the X coordinate as if we had unrolled the helix, but we take the remainder
                        // as a function of the helix circumference (and subtract half so it's displayed in center)
                        object.position.x = ((helix['radius'] * scalefac * angle) % circumference) - circumference/2;
                        object.position.z = i*helix['rise'] * scalefac + helix['offset']*scalefac;

                        this.scene.add( object );
                    }
                }

                this.rotate = false; // we do not want rotation in the planar mode (doesn't make sense to have it)
                this.controls.enableRotate = false; // disable panning with the mouse (doesnt make sense in 2D)

                let axs = new THREE.AxesHelper( 200 );
                axs.position.set(-draw_circumference/2,0,0);
                this.scene.add( axs );
                this.scene.add(this.xaxis)
                this.scene.add(this.yaxis)

                this.camera.position.set( 0, -this.cam_radius, this.cam_height );
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
                //this.scene.background = new THREE.Color( );


                // setup render window
                this.renderer = new THREE.WebGLRenderer( { antialias: true, alpha: true} );
                this.renderer.setPixelRatio( window.devicePixelRatio );
                this.renderer.setSize( this.container.offsetWidth , this.container.offsetHeight );
                let GLwindow = this.container.appendChild( this.renderer.domElement );
                GLwindow.style.display = 'block'; // really important to prevent ghost whitespace below canvas

                this.effect = new OutlineEffect( this.renderer );  // add black outline to helix subunits
                // set up the camera
                THREE.Object3D.DefaultUp = new THREE.Vector3( 0,0,1);
                this.camera = new THREE.PerspectiveCamera( 40, this.container.offsetWidth/this.container.offsetHeight,
                    0.1, 5000 );

                //setup raycaster
                this.raycaster = new THREE.Raycaster();
                this.mouse = new THREE.Vector2();

                // set up the axis overlay (only for 2D mode)
                let points = [];
                points.push( new THREE.Vector3(0,0,0));  // our lines only need 2 vertices
                points.push( new THREE.Vector3(0,0,0));
                const material = new THREE.LineBasicMaterial( { color: 0xd3d3d3 } );
                let geometry = new THREE.BufferGeometry().setFromPoints( points );
                this.yaxis = new THREE.Line( geometry, material );
                this.xaxis = new THREE.Line( geometry.clone(), material );

                // further camera setup
                this.controls = new OrbitControls( this.camera, this.renderer.domElement );
                this.controls.autoRotate = this.rotate;
                this.controls.target.set( 0, 0, this.cam_height );
                this.camera.position.set( this.cam_radius, 0, this.cam_height );
                this.controls.enableKeys = false;
                this.controls.update();

                window.addEventListener( 'resize', () => this.refreshDisplay( this.container ), false );  // detect if page resize
                window.addEventListener( 'mousemove', this.onMouseMove, false );  // handle hover


                // function that is repeatedly automatically called
                const animate = () => {
                    requestAnimationFrame( animate );
                    render();
                };

                const render =  () => {
                    this.controls.update();
                    this.controls.autoRotate = this.rotate;
                    this.effect.render(this.scene, this.camera);
                    this.camera.lookAt(new THREE.Vector3(0, 0, this.cam_height));

                    if (this.planarMode) {  // only do this for the 2D lattice mode
                        this.raycaster.setFromCamera(this.mouse, this.camera);
                        // calculate objects intersecting the picking ray
                        let intersects = this.raycaster.intersectObjects(this.scene.children);

                        if (intersects.length > 0) {
                            this.drawCrosshair(intersects);
                        }
                    }
                };

                console.log('[ Realspace Helix display ready. ]');
                animate()
            },

            drawCrosshair(intersects) {
                const size = 500;
                //  intersects[0].object.material.color.set(0xff0000); // for testing
                const hover_coords = intersects[0].object.position;
                // update horizontal line ('xaxis')
                let xaxis_cor = this.xaxis.geometry.attributes.position.array;
                xaxis_cor[0] = hover_coords.x-size;
                xaxis_cor[2] = xaxis_cor[5] = hover_coords.z;
                xaxis_cor[3] = hover_coords.x+size;
                this.xaxis.geometry.attributes.position.needsUpdate = true;
                // update vertical line ('yaxis')
                let yaxis_cor = this.yaxis.geometry.attributes.position.array;
                yaxis_cor[2] = hover_coords.z-size;
                yaxis_cor[0] = yaxis_cor[3] = hover_coords.x;
                yaxis_cor[5] = hover_coords.z+size;
                this.yaxis.geometry.attributes.position.needsUpdate = true;
            },

            refreshDisplay(container) {  //
                console.log('Refreshing aspect ratio plot');
                this.camera.aspect = container.offsetWidth / container.offsetHeight ;

                this.camera.updateProjectionMatrix();
                this.renderer.setSize( container.offsetWidth, container.offsetHeight  );
            },

            onMouseMove( event ) {
                // calculate mouse position in normalized device coordinates
                // (-1 to +1) for both components
                this.mouse.x = ( event.offsetX / this.renderer.domElement.clientWidth ) * 2 - 1;
                this.mouse.y = - ( event.offsetY / this.renderer.domElement.clientHeight ) * 2 + 1;
            },

            toggleFullscreen(){
                this.fullscreen = !this.fullscreen;

                let card = document.querySelector( '.ui-realspace-panel' );

                if (this.fullscreen) {
                    console.log('[realspace helix] going fullscreen');
                    card.style.position = 'absolute';
                    card.style.width = '90vw';
                    card.style.height = '80vh';  // bit of a bodge
                } else {
                    console.log('[realspace helix] leaving fullscreen');
                    card.style.position = 'static';
                    card.style.width = '0px';
                    card.style.height = 'auto';
                    this.planarMode = false;  // reset to the helical mode. (not 2D lattice)
                }
                this.refreshDisplay(this.container)
            }
        },
        mounted() {
            this.setupDisplay( '.helix-display' )
        }
    }
</script>

<style>
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
        bottom: 0;
        z-index: 2;
        align-items: flex-end;
    }

    .helix-display canvas{
        position:absolute;
        top:0;
        left:0;
        z-index: 1;
        outline: none
    }

    .top-realspace-buttons {
        position: absolute;
        top: 0;
        right: 0;
        padding: 1rem;
        z-index: 2;
    }

    .top-realspace-item {
        margin-left: 1rem;
    }

    @media only screen and (max-width: 600px) {
        .button-fullscreen{
            display: none;
        }

        .helix-display canvas{
            pointer-events: none;
        }
    }
</style>

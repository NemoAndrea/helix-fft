<template>
        <div class="helix-display">
        </div>
</template>

<script>
    import * as THREE from '../../ext/three.module.js';

    export default {
        name: "HelixDisplay",
        props: {
            radius: {
                type: Number,
                default: -1
            },
            rise: {
                type: Number,
                default: -1
            },
            frequency: {
                type: Number,
                default: -1
            },
            unit_size: {
                type: Number,
                default: -1
            }
        },
        watch: {
            // eslint-disable-next-line no-unused-vars
            radius (val){
                this.refreshHelix()
            },
            // eslint-disable-next-line no-unused-vars
            rise (val){
                this.refreshHelix()
            },
            // eslint-disable-next-line no-unused-vars
            frequency (val){
                this.refreshHelix()
            },
            // eslint-disable-next-line no-unused-vars
            unit_size (val){
                this.refreshHelix()
            }
        },
        data: () => ({
            scene: '',
            geometry: '',
            material: '',
        }),
        methods:{
            refreshHelix(){
                console.log('redrawing the helix (three.js)')

                // clear the old helix
                while(this.scene.children.length > 0){
                    this.scene.remove(this.scene.children[0]);
                }

                // compute relevant values
                const scalefac = 10/this.radius;  // the prefactor is an empirical value and might need to be made more robust
                const amount = Math.ceil(100 / (this.rise*scalefac));  // number of helical units to draw
                this.geometry = new THREE.SphereGeometry( this.unit_size*scalefac, 12,12);

                // build the helix
                for ( var i = 0; i < amount ; i ++ ) {
                    let object = new THREE.Mesh( this.geometry, this.material );
                    object.position.x = this.radius*scalefac*Math.cos(i*2*Math.PI/this.frequency);
                    object.position.y = this.radius*scalefac*Math.sin(i*2*Math.PI/this.frequency);
                    object.position.z = i*this.rise*scalefac;


                    this.scene.add( object );
                }

            },

            setupDisplay(){
                // get the div we will draw in
                let container = document.querySelector('.helix-display' );
                let containerinfo = container.getBoundingClientRect();

                // set up the camera
                THREE.Object3D.DefaultUp = new THREE.Vector3( 0,0,1);
                let camera = new THREE.PerspectiveCamera( 40, containerinfo.width/containerinfo.height, 0.1, 15000 );
                camera.position.z = 50;
                const cam_radius = 160;
                camera.position.x = cam_radius;
                let cam_angle = 0;  // initialise to 0

                this.scene = new THREE.Scene();

                // set up helical building blocks
                this.material = new THREE.MeshNormalMaterial();

                // setup render window
                let renderer = new THREE.WebGLRenderer( { antialias: true, alpha: true} );
                renderer.setPixelRatio( window.devicePixelRatio );
                renderer.setSize( containerinfo.width, containerinfo.height );
                container.appendChild( renderer.domElement );

                // function that is repeatedly automatically called
                const animate = () => {
                    requestAnimationFrame( animate );
                    render();
                };

                const render =  () => {
                    camera.position.x = cam_radius*Math.cos(cam_angle);
                    camera.position.y = cam_radius*Math.sin(cam_angle);
                    cam_angle = cam_angle + 0.01;

                    camera.lookAt( new THREE.Vector3(0,0,50) );
                    renderer.render( this.scene, camera );

                };
                animate()
            }
        },
        mounted() {
            this.setupDisplay()
        }
    }
</script>

<style scoped>
    .helix-display{
        height: 100%;
        width: 100%;
    }
</style>

<template>
    <div class="helix-display">
        <div class="realspace-controls">Test</div>
    </div>
</template>

<script>
    import * as THREE from '../../ext/three.module.js';

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
            material: '',
            rotationRate: 0.01,
        }),
        methods:{
            refreshHelix(){
                console.log('redrawing the helix (three.js)');

                // clear the old helices
                while(this.scene.children.length > 0){
                    this.scene.remove(this.scene.children[0]);
                }

                let helix = {};

                // compute relevant scale to draw helix at
                let max_radius=this.helixFamily[0]['radius'];
                for (helix of this.helixFamily) {
                    if ( 1/helix['radius'] < 1/max_radius ){
                        max_radius = helix['radius']
                    }
                }
                const scalefac = 10/max_radius;  // the prefactor is an empirical value and might need to be made more robust

                for (helix of this.helixFamily) {
                    const amount = Math.ceil( 100 / ( helix['rise']*scalefac ) );  // number of helical units to draw
                    let geometry = new THREE.SphereGeometry( helix['unit_size']*scalefac, 12,12 );

                    // build the helix
                    for ( var i = 0; i < amount ; i ++ ) {
                        let object = new THREE.Mesh( geometry, this.material );
                        object.position.x = helix['radius'] * scalefac * Math.cos( i*2*Math.PI / helix['frequency'] );
                        object.position.y = helix['radius'] * scalefac * Math.sin( i*2*Math.PI / helix['frequency'] );
                        object.position.z = i*helix['rise'] * scalefac;


                        this.scene.add( object );
                    }
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
                    cam_angle = cam_angle + this.rotationRate;

                    camera.lookAt( new THREE.Vector3(0,0,50) );
                    renderer.render( this.scene, camera );
                };

                console.log('Realspace Helix display ready.');
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
        position: relative;
    }

    .realspace-controls{
        position: absolute;
        bottom: 10px
    }
</style>

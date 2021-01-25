<template>
    <div class="image-viewer">
        <div style="font-size: 4rem">Helixiser Figure {{ $route.params.id }}</div>
        <div>running from node: {{usingNode}}</div>
        <fourier-panel></fourier-panel>

    </div>
</template>

<script>
    import FourierPanel from "../components/FourierPanel";
    export default {
        name: "ImageViewer",
        components: {FourierPanel},
        data: () => ({
            usingNode: false,
        }),
        mounted() {
            console.log('Imageviewer mounted');
            // set the page title
            document.title = 'Fig ' + this.$route.params.id + ' - Helixiser';
            // check if we are running helixiser locally using Node.js
            if(typeof process === 'object') {
                console.log('[Node] Running from Node');
                this.usingNode = true;
            } else {  // running from browser,
                this.usingNode = false;
            }
        }
    }
</script>

<style scoped>
    .image-viewer {
        margin: 3rem;
    }

</style>

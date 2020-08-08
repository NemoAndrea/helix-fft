<template>
  <v-app>
    <v-main>
      <div class="layout-box">
        <header >Helical FFT</header>
        <div class="layout-left"><div class="rotated">Built by Nemo Andrea</div></div>
        <div class="layout-central">
          <div class="ui-box ui-command-panel">
            <div class="card card-accent">Metadata card |
              Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et
              dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip
              ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu
              fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt
              mollit anim id est laborum</div>
            <div class="card">Helix parameters</div>
          </div>
          <div class="ui-box ui-realspace-panel"><div class="card card-webgl">Realspace helix</div></div>
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

export default {
  name: 'App'
  ,

  data: () => ({
    //
  }),
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

  .card-accent{
    background-color: var(--primary);
    margin-bottom: 1rem;
    color: white;
  }

  .card-webgl{
    border-radius: 7px 7px 7px 7px;
    height: 100%;
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

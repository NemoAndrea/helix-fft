import Vue from 'vue'
import VueRouter from 'vue-router'
import App from './App.vue'
import vuetify from './plugins/vuetify';

import Interface from "./components/Interface";
import ImageViewer from "./components_node/ImageViewer";

Vue.config.productionTip = false;

Vue.use(VueRouter);

const routes = [
  { path: '/', component: Interface, name: 'promotional' },
  { path: '/figure/:id', component: ImageViewer },
];

const router = new VueRouter({
  routes // short for `routes: routes`
});

new Vue({
  vuetify,
  router,
  render: h => h(App)
}).$mount('#app')

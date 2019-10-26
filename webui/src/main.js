import Vue from 'vue'

import App from './App.vue'
import router from './router'
import store from './store'

import BootstrapVue from 'bootstrap-vue'
import 'bootstrap/dist/css/bootstrap.css'
import 'bootstrap-vue/dist/bootstrap-vue.css'
Vue.use(BootstrapVue)

import { Promised } from 'vue-promised'
Vue.component('Promised', Promised)

export const LUCID_SERVER_URI = process.env.LUCID_SERVER_URI || 'http://localhost:7091'

Vue.config.productionTip = false

new Vue({
  router,
  store,
  render: h => h(App)
}).$mount('#app')

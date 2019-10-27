import Vue from 'vue'

import App from './App.vue'
import router from './router'
import store from './store'

// Import Bootstrap + Bootstrap-Vue
import BootstrapVue from 'bootstrap-vue'
import 'bootstrap/dist/css/bootstrap.css'
import 'bootstrap-vue/dist/bootstrap-vue.css'
Vue.use(BootstrapVue)

// Import custom Bootstrap skin
import './main.css'

// Import Vue-Promised
import { Promised } from 'vue-promised'
Vue.component('Promised', Promised)

Vue.config.productionTip = false

new Vue({
  router,
  store,
  render: h => h(App)
}).$mount('#app')

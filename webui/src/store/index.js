import Vue from 'vue'
import Vuex from 'vuex'
import createPersistedState from 'vuex-persistedstate'

import router from '../router'

Vue.use(Vuex)

const defaultState = () => JSON.parse(JSON.stringify({
  token: null
}))

export default new Vuex.Store({
  state: {
    token: null
  },

  actions: {},

  mutations: {
    setLoggedIn(state, token) {
      state.token = token
    },
    setLoggedOut(state) {
      const getDefault = defaultState()
      state.token = getDefault.token
      router.push({ name: 'Home' })
    }
  },

  plugins: [createPersistedState({ key: 'lucid-webui-state' })]
})

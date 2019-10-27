import Vue from 'vue'
import Vuex from 'vuex'
import createPersistedState from 'vuex-persistedstate'

import router from '@/router'
import { checkLucidEndpoint, checkLucidToken } from '@/lucidApi'

Vue.use(Vuex)

const defaultState = () => JSON.parse(JSON.stringify({
  token: null,
  endpoint: {
    rememberEndpoint: null,
    apiUri: null,
    version: null
  }
}))

export default new Vuex.Store({
  state: {
    token: null,
    endpoint: {
      rememberEndpoint: null,
      apiUri: null,
      version: null
    }
  },

  actions: {
    async setEndpoint({ commit }, { endpoint, rememberEndpoint }) {
      // Check the provided endpoint
      const version = await checkLucidEndpoint(endpoint)
      commit('setLucidEndpoint', { endpoint, version, rememberEndpoint })
    },

    async logIn({ commit }, token) {
      // Check the provided token
      await checkLucidToken(token)

      commit('setLoggedIn', token)
      router.push({ name: 'Home' })
    },

    logOut({ commit }) {
      commit('setLoggedOut')
      router.push({ name: 'Login' })
    }
  },

  mutations: {
    setLucidEndpoint(state, { endpoint, version, rememberEndpoint }) {
      state.endpoint.apiUri = endpoint
      state.endpoint.version = version
      state.endpoint.rememberEndpoint = rememberEndpoint
    },
    setLoggedIn(state, token) {
      state.token = token
    },
    setLoggedOut(state) {
      const getDefault = defaultState()
      state.token = getDefault.token
      // Remove endpoint if not should not be remembered
      if (!state.endpoint.rememberEndpoint) {
        state.endpoint.apiUri = getDefault.endpoint.apiUri
        state.endpoint.version = getDefault.endpoint.version
        state.endpoint.rememberEndpoint = getDefault.endpoint.rememberEndpointi
      }
    }
  },

  getters: {
    // Lucid API version
    LUCID_VERSION(state) {
      return state.endpoint.version
    },
    // Lucid API endpoint
    LUCID_API_ENDPOINT(state) {
      return state.endpoint.apiUri
    },
    // Lucid UI endpoint
    LUCID_UI_ENDPOINT(state) {
      return `${state.endpoint.apiUri}/ui`
    },
    // Lucid kv endpoint
    LUCID_KV_ENDPOINT(state) {
      return `${state.endpoint.apiUri}/kv`
    },
    // Check the user is logged in
    isLoggedIn(state) {
      return !!state.token
    }
  },

  // Save Vuex state in LocalStorage
  plugins: [createPersistedState({ key: 'lucid-webui-state' })]
})

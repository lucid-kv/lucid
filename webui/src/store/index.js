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
  },
  kv: {
    isLoading: false
  }
}))

export default new Vuex.Store({
  state: {
    token: null,
    endpoint: {
      rememberEndpoint: null,
      apiUri: null,
      version: null
    },
    kv: {
      isLoading: false
    }
  },

  actions: {
    // Check the Lucid API endpoint and token are still valid after page refresh
    async pageRefreshCheck({ state, commit }) {
      if (!state.token || !state.endpoint.apiUri) return
      try {
        const version = await checkLucidEndpoint(state.endpoint.apiUri)
        // Update the Lucid endpoint version
        commit('setLucidEndpoint', { version, endpoint: state.endpoint.apiUri, rememberEndpoint: state.endpoint.rememberEndpoint })
        await checkLucidEndpoint(state.endpoint.apiUri)
        await checkLucidToken(state.token)
      }
      catch (error) {
        // One of the checks failed, clear the store and redirect with error
        commit('setLoggedOut')
        router.push({ name: 'Login', query: { error: `[Lucid API endpoint check] ${error.message}` } })
      }
    },

    // Check then set the Lucid API endpoint
    async setEndpoint({ commit }, { endpoint, rememberEndpoint }) {
      // Check the provided endpoint
      const version = await checkLucidEndpoint(endpoint)
      commit('setLucidEndpoint', { endpoint, version, rememberEndpoint })
    },

    // Check then set Lucid JWT
    async logIn({ commit }, token) {
      // Check the provided token
      await checkLucidToken(token)

      commit('setLoggedIn', token)
      router.push({ name: 'Home' })
    },

    // Logout
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
    },
    // Register that the kv PoC is in a loading state or not
    setKvLoading(state, isLoading) {
      state.kv.isLoading = isLoading
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
      return !!state.token && !!state.endpoint.apiUri
    },
    // Check the kv PoC is in a loading state
    isKvLoading(state) {
      return state.kv.isLoading
    }
  },

  // Save Vuex state in LocalStorage
  plugins: [createPersistedState({
    key: 'lucid-webui-state',
    reducer: state => ({
      token: state.token,
      endpoint: state.endpoint
    })
  })]
})

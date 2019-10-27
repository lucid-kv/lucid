import Vue from 'vue'
import Vuex from 'vuex'
import createPersistedState from 'vuex-persistedstate'

import router from '@/router'
import { checkLucidToken } from '@/lucidApi'

Vue.use(Vuex)

const defaultState = () => JSON.parse(JSON.stringify({
  token: null
}))

export default new Vuex.Store({
  state: {
    token: null
  },

  actions: {
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
    setLoggedIn(state, token) {
      state.token = token
    },
    setLoggedOut(state) {
      const getDefault = defaultState()
      state.token = getDefault.token
    }
  },

  getters: {
    isLoggedIn(state) {
      return !!state.token
    }
  },

  plugins: [createPersistedState({ key: 'lucid-webui-state' })]
})

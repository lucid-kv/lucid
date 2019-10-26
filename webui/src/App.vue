<template>
  <div id="app">
    <b-navbar toggleable="lg" type="dark" variant="dark" class="mb-2">
      <div class="container">
        <b-navbar-brand :to="{ name: 'Home' }">Lucid</b-navbar-brand>

        <b-navbar-toggle target="nav-collapse" />

        <b-collapse id="nav-collapse" is-nav>
          <b-navbar-nav>
            <b-nav-item :to="{ name: 'Home' }">Home</b-nav-item>
            <b-nav-item v-if="isLoggedIn" :to="{ name: 'KvProofOfConcept' }">KV PoC</b-nav-item>
          </b-navbar-nav>

          <b-navbar-nav class="ml-auto">
            <b-nav-item v-if="!isLoggedIn" :to="{ name: 'Login' }">Log in</b-nav-item>
            <b-nav-item v-else @click="logOut">Log out</b-nav-item>
          </b-navbar-nav>
        </b-collapse>
      </div>
    </b-navbar>

    <div class="container">
      <router-view />
    </div>
  </div>
</template>

<script>
import { mapActions, mapGetters } from 'vuex'

export default {
  computed: {
    ...mapGetters(['isLoggedIn'])
  },
  methods: {
    ...mapActions(['logOut'])
  }
}
</script>

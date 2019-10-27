<template>
  <div id="app">
    <Sidebar>
      <router-view />
    </Sidebar>
  </div>
</template>

<script>
import { mapActions, mapGetters } from 'vuex'

import Sidebar from '@/components/Sidebar'

export default {
  components: {
    Sidebar
  },
  data() {
    return {
      sidebarActive: false
    }
  },
  computed: {
    ...mapGetters(['isLoggedIn'])
  },
  methods: {
    ...mapActions(['logOut'])
  }
}
</script>

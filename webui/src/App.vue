<template>
  <div id="app">
    <transition v-if="pageRefreshCheckLoading" name="fade" mode="out-in">
      <PageLoader text="Checking your Web UI configuration is still valid..." />
    </transition>
    <Sidebar v-else>
      <transition name="fade" mode="out-in">
        <router-view class="my-3" />
      </transition>
    </Sidebar>
  </div>
</template>

<script>
import { mapActions } from 'vuex'

import Sidebar from '@/components/Sidebar'
import PageLoader from '@/components/PageLoader'

export default {
  components: {
    Sidebar,
    PageLoader
  },
  data() {
    return {
      pageRefreshCheckLoading: false
    }
  },
  async mounted() {
    this.pageRefreshCheckLoading = true
    try {
      await this.pageRefreshCheck()
    }
    finally {
      this.pageRefreshCheckLoading = false
    }
  },
  methods: {
    ...mapActions(['pageRefreshCheck'])
  }
}
</script>

<style>
.fade-enter-active, .fade-leave-active {
  transition-duration: 0.2s;
  transition-property: opacity;
  transition-timing-function: ease;
}

.fade-enter, .fade-leave-active {
  opacity: 0
}
</style>

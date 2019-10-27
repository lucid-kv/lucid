<template>
  <div id="app">
    <Sidebar>
      <transition name="fade" mode="out-in">
        <router-view />
      </transition>
    </Sidebar>
  </div>
</template>

<script>
import Sidebar from '@/components/Sidebar'

export default {
  components: {
    Sidebar
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

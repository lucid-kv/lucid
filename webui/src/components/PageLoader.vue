<template>
  <div v-if="show" class="full">
    <div class="overlay"></div>
    <div id="spinner">
      <Loader />
    </div>
  </div>
</template>

<script>
import Loader from '@/components/Loader'

export default {
  components: {
    Loader
  },
  data() {
    return {
      show: true
    }
  },
  methods: {
    showLoader(showIt = true) {
      this.show = showIt
    }
  }
}
</script>

<style scoped>
.full {
  display: flex;
  position: fixed;
  width: 100%;
  height: 100%;
  top: 0;
  left: 0;
  margin: 0 auto !important;
  text-align: center !important;
  z-index: 999;
}

#spinner {
  width: 100% !important;
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
  width: fit-content;
  z-index: 99;
  color: white;
}

.overlay {
  opacity: 1;
  width: 100%;
  height: 100%;
  position: absolute;
  z-index: 98;
  background-color: #1f274491;
  backdrop-filter: blur(4px);
}
</style>

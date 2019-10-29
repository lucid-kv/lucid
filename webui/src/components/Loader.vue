<template>
  <div class="my-3" :class="{ center }">
    <div id="spinner">
      <feather type="triangle" animation="spin" animation-speed="fast" />
      <p>{{ text }}</p>
    </div>
  </div>
</template>

<script>
export default {
  props: {
    text: {
      type: String,
      required: false,
      default: 'Loading...'
    },
    center: {
      type: Boolean,
      required: false,
      default: false
    }
  }
}
</script>

<style scoped>
#spinner {
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
  margin: 20px 0px;
  width: fit-content;
}
#spinner p {
  margin-top: 5px;
}

.center {
  margin: 0 auto !important;
  text-align: center !important;
}
.center #spinner {
  width: 100% !important;
}
</style>

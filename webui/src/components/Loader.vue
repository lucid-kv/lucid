<template>
  <span id="spinner">
    <feather type="triangle" animation="spin" animation-speed="fast" />
    <p>Loading...</p>
  </span>
</template>

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
</style>

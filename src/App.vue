<script setup lang="ts">
import Wizard from "./components/Wizard.vue";
</script>

<template>
  <div class="container">
    <h1>PrintNanny Imager</h1>

    <div class="row">
      <a href="https://printnanny.ai" target="_blank">
        <img src="./assets/logo.svg" class="logo vite" alt="PrintNanny Logo" />
      </a>
    </div>

    <Wizard />
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>

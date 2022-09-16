<template>
  <div
    v-if="store.progress"
    class="text-center grid grid-rows-2 items-center w-full"
  >
    <p class="w-full text-white">
      {{ store.progress?.label }} ({{ store.progress?.elapsed }} seconds
      elapsed)
    </p>
    <div
      class="w-3/4 bg-gray-200 rounded-full dark:bg-gray-700 m-auto text-center"
    >
      <div
        class="bg-indigo-400 text-xs font-medium text-indigo-100 text-center p-0.5 leading-none rounded-full"
        :style="{ width: store.progress.display_percent() }"
      >
        {{ store.progress.display_percent() }}
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { useStore } from "@/store";

const store = useStore();
</script>

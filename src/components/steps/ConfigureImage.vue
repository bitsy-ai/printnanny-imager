<template>
  <div v-if="!active" class="flex-1">
    <button
      disabled
      class="text-center block mx-4 my-4 h-12 w-48 block bg-indigo-400 text-white font-bold py-2 px-4 border-b-4 border-indigo-700 rounded disabled:opacity-50"
    >
      Configure Image
    </button>
  </div>
  <div v-else class="flex-1">
    <button
      class="text-center block mx-4 my-4 h-12 w-48 block bg-indigo-400 hover:bg-indigo-500 text-white font-bold py-2 px-4 border-b-4 border-indigo-700 hover:border-indigo-600 rounded"
      @click="onClick"
    >
      Configure Image
    </button>
  </div>
</template>
<script setup lang="ts">
import { computed } from "vue";
import { useRouter } from "vue-router";
import { open } from "@tauri-apps/api/dialog";
import { useStore } from "@/store";

const router = useRouter();
const store = useStore();
const key = "configure-image";
const active = computed(() => router.currentRoute.value.name == key);

function onClick(){}
function clearSelection() {
  store.$patch({ selectedImageFile: null, selectedDisk: null });
  router.push({ name: "select-image" });
}
</script>

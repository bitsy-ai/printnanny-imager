<template>
  <div v-if="!active" class="flex-1">
    <p class="text-center text-stone-50 text-sm truncate">
      {{ imageWriterStatus }}
    </p>
    <button
      disabled
      class="text-center block mx-4 my-4 h-12 w-48 block bg-indigo-400 text-white font-bold py-2 px-4 border-b-4 border-indigo-700 rounded disabled:opacity-50"
    >
      Flash!
    </button>
  </div>
  <div v-else class="flex-1 h-20">
    <p class="text-center text-stone-50 text-sm truncate">
      {{ imageWriterStatus }}
    </p>
    <button
      class="text-center block mx-4 my-4 h-12 w-48 block bg-indigo-400 hover:bg-indigo-500 text-white font-bold py-2 px-4 border-b-4 border-indigo-700 hover:border-indigo-600 rounded"
      @click="onClick"
    >
      Flash!
    </button>
  </div>
</template>
<script setup lang="ts">
import { computed } from "vue";
import { useRouter } from "vue-router";
import { useStore } from "@/store";
import { flashImage } from "@/utils/disk";

const store = useStore();
const router = useRouter();
const key = "flash-image";
const active = computed(() => router.currentRoute.value.name == key);

const imageWriterStatus = computed(() => {
  if (active.value == true) {
    return "Click Flash! to write image";
  }
  return "";
});

async function onClick() {
  if (store.selectedDisk == null) {
    console.error("Failed to flash, selectedDisk is null");
  } else if (store.selectedImageFile == null) {
    console.error("Failed to flash, selectedImageFile is null");
  } else {
    console.log(
      `Flashing ${store.selectedImageFile} to ${store.selectedDisk.path}`
    );
    await flashImage(store.selectedDisk, store.selectedImageFile);
  }
}
</script>

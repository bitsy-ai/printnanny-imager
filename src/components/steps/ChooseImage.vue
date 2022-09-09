<template>
  <div v-if="store.selectedImageFile" class="flex-1 justify-items-center">
    <p class="text-center text-stone-50 text-sm truncate">
      {{ truncate(filename(store.selectedImageFile)) }}
    </p>
    <button
      class="text-center block mx-4 my-4 h-12 w-48 block bg-indigo-500 hover:bg-indigo-400 text-white font-bold py-2 px-4 border-b-4 border-indigo-700 hover:border-indigo-500 rounded"
      @click="clearSelection"
    >
      Clear selection
    </button>
  </div>
  <div v-else class="flex-1">
    <!--
        <a href="#" class="mx-4 my-4 text-center relative h-12 w-48 block bg-indigo-500 hover:bg-indigo-400 text-white font-bold py-2 px-4 border-b-4 border-indigo-700 hover:border-indigo-500 rounded">
            PrintNanny OS
        </a>
        -->
    <button
      class="text-center block mx-4 my-4 h-12 w-48 block bg-indigo-500 hover:bg-indigo-400 text-white font-bold py-2 px-4 border-b-4 border-indigo-700 hover:border-indigo-500 rounded"
      @click="openFile"
    >
      Flash from file
    </button>
  </div>
</template>
<script setup lang="ts">
import { useRouter } from "vue-router";
import { open } from "@tauri-apps/api/dialog";
import { useStore } from "@/store";

const router = useRouter();
const store = useStore();

function truncate(str: string): string {
  return str.slice(0, 10) + "..." + str.slice(str.length - 10, str.length);
}
function filename(path: string): string {
  return path.split("\\").pop().split("/").pop() || "";
}

async function openFile() {
  const selected = await open({
    multiple: false,
    filters: [
      {
        name: "Image",
        extensions: ["wic", "zip", "tar.gz", "img"],
      },
    ],
  });
  if (selected) {
    store.$patch({ selectedImageFile: selected });
    router.push({ name: "select-storage" });
  }
}

function clearSelection() {
  store.$patch({ selectedImageFile: null, selectedDisk: null });
  router.push({ name: "select-image" });
}
</script>

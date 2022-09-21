<template>
  <div v-if="store.selectedImageFile" class="flex-1 justify-items-center h-20">
    <button
      class="text-center block mx-4 my-4 h-12 w-48 block bg-indigo-400 hover:bg-indigo-500 text-white font-bold py-2 px-4 border-b-4 border-indigo-700 hover:border-indigo-600 rounded"
      @click="clearSelection"
    >
      Clear Selection
    </button>
    <p class="text-center text-stone-50 text-sm truncate">
      {{ truncate(filename(store.selectedImageFile)) }}
    </p>
  </div>
  <div v-else class="flex-1">
    <button
      class="text-center block mx-4 my-4 h-12 w-48 block bg-indigo-400 hover:bg-indigo-500 text-white font-bold py-2 px-4 border-b-4 border-indigo-700 hover:border-indigo-600 rounded"
      @click="openFile"
    >
      Flash from File
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
  const result = path.split("\\");
  if (result !== undefined) {
    return result?.pop()?.split("/").pop() || "";
  }
  return path;
}

async function openFile() {
  const selected = (await open({
    multiple: false,
    filters: [
      {
        name: "Image",
        extensions: ["wic", "zip", "tar.gz", "img", "iso"],
      },
    ],
  })) as string | null;
  if (selected !== null) {
    store.$patch({ selectedImageFile: selected });
    router.push({ name: "configure-image" });
  }
}

function clearSelection() {
  store.$patch({ selectedImageFile: null, selectedDisk: null });
  router.push({ name: "select-image" });
}
</script>

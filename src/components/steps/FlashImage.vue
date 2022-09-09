<template>
    <div v-if="!active" class="flex-1">
        <p class="text-center text-stone-50 text-sm truncate">{{imageWriterStatus}}</p>
        <button disabled class="text-center block mx-4 my-4 h-12 w-48 block bg-indigo-500 text-white font-bold py-2 px-4 border-b-4 border-indigo-700 rounded disabled:opacity-50">
            Flash!
        </button>  
    </div>
    <div v-else class="flex-1">
        <p class="text-center text-stone-50 text-sm truncate">{{ imageWriterStatus}}</p>
        <button 
        @click="onClick" 
        class="text-center block mx-4 my-4 h-12 w-48 block bg-indigo-500 hover:bg-indigo-400 text-white font-bold py-2 px-4 border-b-4 border-indigo-700 hover:border-indigo-500 rounded">
        Flash!
        
        </button>
    </div>
</template>
<script setup lang="ts">
import { computed, ref } from "vue";
import { useRouter } from "vue-router";
import { useStore } from "@/store";
import { flashImage } from "@/utils/disk";

const store = useStore();
const router = useRouter();
const key = "flash-image";
const active = computed(() => router.currentRoute.value.name == key);

const imageWriterStatus = ref('');
if (active.value == true){
    imageWriterStatus.value = 'Click Flash! to write image'
}

async function onClick(){
    if (store.selectedDisk == null){
        console.error("Failed to flash, selectedDisk is null");
    } else if (store.selectedImageFile == null){
        console.error("Failed to flash, selectedImageFile is null");
    }
    else {
        console.log(`Flashing ${store.selectedImageFile} to ${store.selectedDisk.path}`)
        await flashImage(store.selectedDisk, store.selectedImageFile);
    }
}

</script>

<template>
    <div v-if="!active" class="flex-1">
        <button disabled class="text-center block mx-4 my-4 h-12 w-48 block bg-indigo-500 text-white font-bold py-2 px-4 border-b-4 border-indigo-700 rounded disabled:opacity-50">
            Flash!
        </button>  
    </div>
    <div v-else class="flex-1">
        <button 
        @click="onClick" 
        class="text-center block mx-4 my-4 h-12 w-48 block bg-indigo-500 hover:bg-indigo-400 text-white font-bold py-2 px-4 border-b-4 border-indigo-700 hover:border-indigo-500 rounded">
        {{ store.selectedDisk !== undefined ? `Clear selection` :'Choose Storage'}}
        
        </button>
    </div>
</template>
<script setup lang="ts">
import { computed } from "vue";
import { useRouter } from "vue-router";
import { useStore } from "@/store";

const store = useStore();
const router = useRouter();
const key = "flash-image";
const active = computed(() => router.currentRoute.value.name == key)

</script>

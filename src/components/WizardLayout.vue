<template>
<div class="flex grid grid-rows-3 max-h-full">
    <div class="flex-1 row-span-1 bg-stone-50">
        <img
          src="@/assets/logo/logo-rect-light.svg"
          class="w-1/2 m-auto pt-2 pb-2"
          alt="PrintNanny Logo"
        />

    </div>

    <div class="row-span-2 bg-indigo-400">
        <div class="grid grid-cols-2 gap-4 flex flex-1 justify-items-center text-base">
            <!-- <CompactSelect class="mt-4" store-attr="board" :options="Object.values(boards)" label="Hardware: "/> -->
            <CompactSelect class="mt-4" store-attr="os" :options="Object.values(operatingSystems)" label="Operating System: "/>

            <StorageSelect class="mt-4"/>
            <div  class="flex-1">
                <p class="text-stone-50 prose prose-xl text-center">Configure</p>
                <button class="bg-stone-300 hover:bg-stone-200 text-stone-900 font-bold py-2 px-4 border-b-4 border-stone-500 hover:border-stone-300 rounded">Configure System with Cloud-Init</button>
            </div>
        </div>
        <div class="flex flex-1 row-span-1 bg-indigo-400 min-h-full w-full text-base items-center">
            <div class="flex-1 items-center justify-items-center">
                <p class="text-stone-50 prose prose-xl text-center m-auto">Write</p>
                <button class="m-auto block bg-stone-300 hover:bg-stone-200 text-stone-900 font-bold py-2 px-4 border-b-4 border-stone-500 hover:border-stone-300 rounded">Write to SD</button>
            </div>
        </div>
    </div>
</div>
</template>
<script setup lang="ts">
import { FolderDownloadIcon } from "@vue-hero-icons/outline"
import CompactSelect from './select/CompactSelect.vue'
import StorageSelect from './select/StorageSelect.vue';
import boards from "@/data/boards";
import operatingSystems from "@/data/os";

import { useWizardStore } from "@/store/wizard";

const store = useWizardStore();
store.listRemoveableDrives()

function reloadDisks(){
    console.log("Reloading disks")
}

</script>
<template>
    <div class="flex grid grid-rows-4 max-h-full">
        <div class="flex-1 row-span-1 bg-stone-50">
            <img
              src="@/assets/logo/logo-rect-light.svg"
              class="w-1/2 m-auto pt-2 pb-2"
              alt="PrintNanny Logo"
            />
    
        </div>
    
        <div class="row-span-3 bg-zinc-500">
            <div class="grid grid-rows-2">
                <nav aria-label="Progress">
                    <ol role="list" class="flex items-center mt-6 justify-center">
                    <li v-for="(step, stepIdx) in steps" :key="step.name" :class="[stepIdx !== steps.length - 1 ?  'pr-56' : '', 'relative']">
                        <template v-if="step.status === 'complete'">
                        <div class="absolute inset-0 flex items-center justify-center" aria-hidden="true">
                            <div class="h-0.5 w-full bg-indigo-600" />
                        </div>
                        <a href="#" class="relative flex h-12 w-12 items-center justify-center rounded-full bg-indigo-600 hover:bg-indigo-900">
                            <component :is="step.solidIcon" class="h-8 w-8 text-white"></component>
                            <span class="sr-only">{{ step.name }}</span>
                        </a>
                        </template>
                        <template v-else-if="step.status === 'current'" condition="step.status === 'current'">
                        <div class="absolute inset-0 flex items-center" aria-hidden="true">
                            <div class="h-0.5 w-full bg-gray-200" />
                        </div>
                        <a href="#" class="relative flex h-12 w-12 items-center justify-center rounded-full border-2 border-indigo-600 bg-white" aria-current="step">
                            <component :is="step.solidIcon" class="h-8 w-8 text-indigo-600"></component>
                            <span class="sr-only">{{ step.name }}</span>
                        </a>
                        </template>
                        <template v-else>
                        <div class="absolute inset-0 flex items-center" aria-hidden="true">
                            <div class="h-0.5 w-full bg-gray-200" />
                        </div>
                        <a href="#" class="group relative flex h-12 w-12 items-center justify-center rounded-full border-2 border-gray-300 bg-white hover:border-gray-400">
                            <component :is="step.solidIcon" class="h-8 w-8 text-gray-400"></component>
                            <span class="sr-only">{{ step.name }}</span>
                        </a>
                        </template>
                    </li>
                    </ol>
                    <div class="flex items-center mt-6 justify-space-around grid grid-cols-3">
                        <ChooseImage />
                        <ChooseDisk />
                        <FlashButton />
                    </div>

                </nav>
            </div>
        </div>
    </div>
</template>
<script setup lang="ts">
import { RouterView } from "vue-router";
import { FolderArrowDownIcon as FolderArrowDownIconOutline, ServerIcon as ServerIconOutline, BoltIcon as BoltIconOutline } from "@heroicons/vue/24/outline";
import { FolderArrowDownIcon as FolderArrowDownIconSolid, ServerIcon as ServerIconSolid, BoltIcon as BoltIconSolid } from "@heroicons/vue/24/solid";

import CompactSelect from './select/CompactSelect.vue'
import StorageSelect from './select/StorageSelect.vue';
import boards from "@/data/boards";
import operatingSystems from "@/data/os";

import ChooseImage from "@/components/steps/ChooseImage.vue";
import ChooseDisk from "@/components/steps/ChooseDisk.vue";
import FlashButton from "@/components/steps/FlashButton.vue";

import { useWizardStore } from "@/store/wizard";


const steps = [
{ name: 'Choose image', href: '#', status: 'complete', outlineIcon: FolderArrowDownIconOutline, solidIcon: FolderArrowDownIconSolid  },
{ name: 'Choose disk', href: '#', status: 'current', outlineIcon: ServerIconOutline, solidIcon: ServerIconSolid },
{ name: 'Flash!', href: '#', status: 'upcoming', outlineIcon: BoltIconOutline, solidIcon: BoltIconSolid },
]
const store = useWizardStore();
// store.listRemoveableDrives()

function reloadDisks(){
    console.log("Reloading disks")
}

</script>
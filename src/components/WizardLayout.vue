<template>
<div class="flex grid grid-rows-3">
    <div class="flex-1 row-span-1 bg-stone-50">
        <img
          src="@/assets/logo/logo-rect-light.svg"
          class="w-3/4 m-auto pt-2 pb-2"
          alt="PrintNanny Logo"
        />

    </div>

    <div class="bg-indigo-400 grid grid-cols-2 gap-4 flex flex-1 justify-items-center text-base">
        <CompactSelect store-attr="board" :options="Object.values(boards)" label="Hardware: "/>
        <CompactSelect store-attr="os" :options="Object.values(operatingSystems)" label="Operating System: "/>


        <div  class="flex-1">
            <p class="text-stone-50 prose prose-xl text-center">Storage</p>
            <button class="bg-stone-300 hover:bg-stone-200 text-stone-900 font-bold py-2 px-4 border-b-4 border-stone-500 hover:border-stone-300 rounded">Choose Storage</button>
        </div>
        <div  class="flex-1">
            <p class="text-stone-50 prose prose-xl text-center">Configure</p>
            <button class="bg-stone-300 hover:bg-stone-200 text-stone-900 font-bold py-2 px-4 border-b-4 border-stone-500 hover:border-stone-300 rounded">Configure System</button>
        </div>
    </div>
    <div class="flex flex-1 row-span-1 bg-indigo-400 min-h-full w-full justify-items-center justify-center text-base items-center">
        <div class="flex-1 items-center justify-items-center">
            <p class="text-stone-50 prose prose-xl text-center">Hardware</p>
            <button class="m-auto bg-stone-300 hover:bg-stone-200 text-stone-900 font-bold py-2 px-4 border-b-4 border-stone-500 hover:border-stone-300 rounded">Write Image</button>
        </div>
    </div>
</div>
</template>
<script setup lang="ts">
import HardwareSelect from './select/HardwareSelect.vue';
import OperatingSystemSelect from './select/OperatingSystemSelect.vue';
import CompactSelect from './select/CompactSelect.vue';
import boards from "@/data/boards";
import operatingSystems from "@/data/os";


</script>
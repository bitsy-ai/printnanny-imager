<template>
<div class="container m-auto">
  <div class="sm:hidden">
    <label for="tabs" class="sr-only">Select configuration mode</label>
    <select id="tabs" class="bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500">
        <option>Basic Settings</option>
        <option>Advanced Settings</option>
    </select>
  </div>
  <ul class="hidden text-sm font-medium text-center text-gray-500 rounded-lg divide-x divide-gray-200 shadow sm:flex dark:divide-gray-700 dark:text-gray-400">
      <li class="w-full">
          <a @click="() => setConfigMode('basic')" class="inline-block p-4 w-full text-gray-900 bg-white rounded-l-lg focus:ring-4 focus:ring-blue-300 focus:outline-none dark:bg-gray-700 dark:text-white" aria-current="page" :class="configMode=='basic' ? 'bg-gray-100' : 'bg-white'">Basic</a>
      </li>
      <li class="w-full">
          <a @click="() => setConfigMode('advanced')" class="inline-block p-4 w-full bg-white rounded-r-lg hover:text-gray-700 hover:bg-gray-50 focus:ring-4 focus:outline-none focus:ring-blue-300 dark:hover:text-white dark:bg-gray-800 dark:hover:bg-gray-700" :class="configMode=='advanced' ? 'bg-gray-100' : 'bg-white'">Advanced</a>
      </li>
  </ul>

  <BasicSettingsForm v-if="configMode == 'basic'"></BasicSettingsForm>
  <AdvancedSettingsForm v-if="configMode == 'advanced'"></AdvancedSettingsForm>
</div>
</template>
<script setup lang="ts">
import { ref} from "vue"
import BasicSettingsForm from "../forms/BasicSettingsForm.vue";
import AdvancedSettingsForm from "../forms/AdvancedSettingsForm.vue";

const configMode = ref('basic');

function setConfigMode(mode: string){
  console.log(mode)
  configMode.value = mode;
}

</script>

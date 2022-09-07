<!-- This example requires Tailwind CSS v2.0+ -->
<template>
<div>
    <nav aria-label="Progress" class="pb-1">
    <ol role="list" class="divide-y divide-gray-300 rounded-md border border-gray-300 md:flex md:divide-y-0">
      <img src="@/assets/logo/logo-rect-light.svg" class="w-1/4 m-auto p-1" alt="PrintNanny Logo" />

      <li v-for="(step, stepIdx) in steps" :key="step.name" class="relative md:flex md:flex-1">
        <router-link v-if="step.completed" :to="step.link" class="group flex w-full items-center">
          <span class="flex items-center px-6 py-4 text-sm font-medium">
            <span class="flex h-10 w-10 flex-shrink-0 items-center justify-center rounded-full bg-indigo-600 group-hover:bg-indigo-800">
              <CheckIcon class="h-6 w-6 text-white" aria-hidden="true" />
            </span>
            <span class="ml-4 text-sm font-medium text-gray-900">{{ step.name }}</span>
          </span>
        </router-link>
        <router-link v-else-if="step.current" :to="step.link"  class="flex items-center px-6 py-4 text-sm font-medium" aria-current="step">
          <span class="flex h-10 w-10 flex-shrink-0 items-center justify-center rounded-full border-2 border-indigo-600">
            <span class="text-indigo-600">{{ step.id }}</span>
          </span>
          <span class="ml-4 text-sm font-medium text-indigo-600">{{ step.name }}</span>
        </router-link>
        <router-link v-else :to="step.link" class="group flex items-center">
          <span class="flex items-center px-6 py-4 text-sm font-medium">
            <span class="flex h-10 w-10 flex-shrink-0 items-center justify-center rounded-full border-2 border-gray-300 group-hover:border-gray-400">
              <span class="text-gray-500 group-hover:text-gray-900">{{ step.id }}</span>
            </span>
            <span class="ml-4 text-sm font-medium text-gray-500 group-hover:text-gray-900">{{ step.name }}</span>
          </span>
        </router-link>
        <template v-if="stepIdx !== steps.length - 1">
          <!-- Arrow separator for lg screens and up -->
          <div class="absolute top-0 right-0 hidden h-full w-5 md:block" aria-hidden="true">
            <svg class="h-full w-full text-gray-300" viewBox="0 0 22 80" fill="none" preserveAspectRatio="none">
              <path d="M0 -2L20 40L0 82" vector-effect="non-scaling-stroke" stroke="currentcolor" stroke-linejoin="round" />
            </svg>
          </div>
        </template>
      </li>
    </ol>
  </nav>
  <RouterView v-slot="{ Component }">
                  <template v-if="Component">
                    <Transition mode="out-in" name="fade">
                      <Suspense>
                        <!-- main content -->
                        <component :is="Component"></component>
                        <!-- loading state -->
                        <template #fallback>
                          Loading...
                        </template>
                      </Suspense>
                    </Transition>
                  </template>
    </RouterView>
</div>
</template>
  
<script setup lang="ts">
import { ref } from "vue"
import {RouterLink} from "vue-router";
import { CheckIcon } from "@heroicons/vue/24/solid";


const steps = ref([
{ id: '1', name: 'Choose Edition', link: {name: "choose-edition"}, completed: false, current: true },
{ id: '2', name: 'Configure', link: {name: "configure-edition", params: {edition: "octoprint"}}, completed: false, current: false },
{ id: '3', name: 'Create SD Card', link: {name: "sd-card"}, completed: false, current: false  },
]);
</script>
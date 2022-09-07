<template>
<main class="flex-1">
    <h2 class="font-medium leading-tight text-3xl text-center p-4">Choose PrintNanny OS Edition</h2>
    <div class="flex grid grid-cols-2 gap-2 p-2">
        <div class="flex flex-1 max-w-full bg-white rounded-lg border border-gray-200 shadow-md dark:bg-gray-800 dark:border-gray-700" v-for="edition in editions" :key="edition.name">
            <div class="p-5">
                <a href="#">
                        <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">{{ edition.name }}</h5>
                    </a>
                    <p class="mb-3 font-normal text-gray-700 dark:text-gray-400">{{ edition.description }}</p>
                    <a v-if="edition.docs" :href="edition.docs" target="_blank" class="inline-flex items-center py-2 px-3 text-sm font-medium text-center text-white bg-zinc-700 rounded-lg hover:bg-zinc-800 focus:ring-4 focus:outline-none focus:ring-zinc-300 dark:bg-zinc-600 dark:hover:bg-zinc-700 dark:focus:ring-zinc-800">
                        Learn More
                        <QuestionMarkCircleIcon class="ml-2 -mr-1 w-4 h-4" ></QuestionMarkCircleIcon>
                    </a>
                    <router-link 
                        :to="edition.select"
                        class="inline-flex items-right py-2 px-3 text-sm font-medium text-center text-white bg-indigo-700 rounded-lg hover:bg-indigo-800 focus:ring-4 focus:outline-none focus:ring-indigo-300 dark:bg-indigo-600 dark:hover:bg-indigo-700 dark:focus:ring-indigo-800">
                        Select
                        <ArrowRightIcon class="ml-2 -mr-1 w-4 h-4" ></ArrowRightIcon>

                    </router-link>

            </div>
        </div>
    </div>
</main>    
</template>
<script setup lang="ts">
import { QuestionMarkCircleIcon, ArrowRightIcon } from "@heroicons/vue/24/solid";

const editions = [
    {name: "OctoPrint", select: {name: "configure-edition", params: {edition: "octoprint"} }, description: "The snappy web interface for your 3D printer.", docs: "https://docs.printnanny.ai/docs/category/quick-start/"},
    {name: "OctoPrint + Klipper", select: {name: "configure-edition", params: {edition: "octoprint-klipper"} }, description: "OctoPrint pre-configured to communicate with Klipper firmware service (Klippy).", docs: "https://docs.printnanny.ai/docs/category/quick-start/"},
    {name: "Mainsail", select: {name: "configure-edition", params: {edition: "mainsail"} }, description: "Mainsail UI, Moonraker API, and Klipper. Everything you need to build+flash Klipper firmware to microcontroller.", docs: "https://docs.printnanny.ai/docs/category/quick-start/"},
    {name: "Custom Image", select: {name: "configure-edition", params: {edition: "custom"} }, description: "Select a custom .img from your computer."}

]
</script>
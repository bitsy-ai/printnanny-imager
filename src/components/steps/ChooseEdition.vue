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
                    <a :href="edition.docs" target="_blank" class="inline-flex items-center py-2 px-3 text-sm font-medium text-center text-white bg-zinc-700 rounded-lg hover:bg-zinc-800 focus:ring-4 focus:outline-none focus:ring-zinc-300 dark:bg-zinc-600 dark:hover:bg-zinc-700 dark:focus:ring-zinc-800">
                        Learn More
                        <svg aria-hidden="true" class="ml-2 -mr-1 w-4 h-4" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z" clip-rule="evenodd"></path></svg>
                    </a>
                    <router-link 
                        :to="edition.select"
                        class="inline-flex items-right py-2 px-3 text-sm font-medium text-center text-white bg-indigo-700 rounded-lg hover:bg-indigo-800 focus:ring-4 focus:outline-none focus:ring-indigo-300 dark:bg-indigo-600 dark:hover:bg-indigo-700 dark:focus:ring-indigo-800">
                        Select

                    </router-link>

            </div>
        </div>
    </div>
</main>    
</template>
<script setup lang="ts">
const editions = [
    {name: "OctoPrint", select: {name: "configure-edition", params: {edition: "octoprint"} }, description: "The snappy web interface for your 3D printer.", docs: "https://docs.printnanny.ai/docs/category/quick-start/"},
    {name: "OctoPrint + Klipper", select: {name: "configure-edition", params: {edition: "octoprint-klipper"} }, description: "OctoPrint pre-configured to communicate with Klipper firmware service (Klippy.)", docs: "https://docs.printnanny.ai/docs/category/quick-start/"},
    {name: "Mainsail", select: {name: "configure-edition", params: {edition: "mainsail"} }, description: "Mainsail UI, Moonraker API, and Klipper. Everything you need to build+flash Klipper firmware to microcontroller.", docs: "https://docs.printnanny.ai/docs/category/quick-start/"}
]
</script>
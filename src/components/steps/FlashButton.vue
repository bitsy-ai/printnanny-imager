<template>
    <a v-if="active" href="#" class="text-center group relative block h-12 w-36 bg-blue-500 hover:bg-blue-400 text-white font-bold py-2 px-4 border-b-4 border-blue-700 hover:border-blue-500 rounded">
        <span>Flash!</span>
    </a>
    <a v-else disabled="true" class="text-center group relative block h-12 w-36 bg-blue-500 hover:bg-blue-400 text-white font-bold py-2 px-4 border-b-4 border-blue-700 hover:border-blue-500 rounded">
        <span>Flash!</span>
    </a>
</template>
<script setup lang="ts">
import { computed } from 'vue';
import { useRouter } from 'vue-router';

const router = useRouter();
const key = "flash";
const active = computed(() => router.currentRoute.value.name == key)

</script>

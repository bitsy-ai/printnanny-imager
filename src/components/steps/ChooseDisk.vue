<template>
    <div v-if="active">
        <a href="#" class="text-center relative h-12 w-36 block bg-blue-500 hover:bg-blue-400 text-white font-bold py-2 px-4 border-b-4 border-blue-700 hover:border-blue-500 rounded">
            Choose Disk
        </a>      
    </div>
    <div v-else>choose disk inactive</div>
</template>
<script setup lang="ts">
import { computed } from 'vue';
import { useRouter } from 'vue-router';

const router = useRouter();
const key = "choose-disk";
const active = computed(() => router.currentRoute.value.name == key);

</script>

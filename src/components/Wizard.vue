<script setup lang="ts">
    import { ref } from "vue";
    import { invoke } from "@tauri-apps/api/tauri";
    
    const greetMsg = ref("");
    const name = ref("");
    
    async function greet() {
      greetMsg.value = await invoke("greet", { name: name.value });
    }
</script>
    
<template>
    <div class="card">
    <h2>Choose an edition:</h2>

    <div class="card">
        <h3>OctoPrint</h3>
        <p>The snappy web interface for your 3D printer.</p>
    </div>
    <div class="card">
        <h3>Mainsail</h3>
        <p>Mainsail UI, Moonraker, and Klipper API are pre-installed.</p>
    </div>
    </div>
</template>

<template></template>
<script setup lang="ts">
import { useWizardStore } from '../../store/wizard';


const store = useWizardStore();
await store.listDisks()
</script>
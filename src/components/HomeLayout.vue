<template>
  <div class="flex grid grid-rows-3 max-h-screen">
    <div class="flex-1 row-span-1 bg-stone-50">
      <StickyAlerts />
      <img
        src="@/assets/logo/logo-rect-light.svg"
        class="m-auto h-full pt-2 pb-2"
        alt="PrintNanny Logo"
      />
    </div>

    <div class="row-span-2 bg-zinc-500">
      <!-- imager buttons -->
      <div
        class="grid grid-rows-3 bg-zinc-500 items-center content-center justify-content-center"
      >
        <nav aria-label="Progress">
          <ol role="list" class="flex items-center mt-6 justify-center">
            <li
              v-for="(step, stepIdx) in steps"
              :key="step.name"
              :class="[stepIdx !== steps.length - 1 ? 'pr-64' : '', 'relative']"
            >
              <template v-if="step.status() === 'complete'">
                <div
                  class="absolute inset-0 flex items-center justify-center"
                  aria-hidden="true"
                >
                  <div class="h-0.5 w-full bg-indigo-400" />
                </div>
                <span
                  class="relative flex h-12 w-12 items-center justify-center rounded-full bg-indigo-400"
                >
                  <component
                    :is="step.solidIcon"
                    class="h-8 w-8 text-white"
                  ></component>
                  <span class="sr-only">{{ step.name }}</span>
                </span>
              </template>
              <template v-else-if="step.status() === 'current'">
                <div
                  class="absolute inset-0 flex items-center"
                  aria-hidden="true"
                >
                  <div class="h-0.5 w-full bg-gray-200" />
                </div>
                <span
                  class="relative flex h-12 w-12 items-center justify-center rounded-full border-2 border-indigo-400 bg-white"
                  aria-current="step"
                >
                  <component
                    :is="step.solidIcon"
                    class="h-8 w-8 text-indigo-400"
                  ></component>
                  <span class="sr-only">{{ step.name }}</span>
                </span>
              </template>
              <template v-else>
                <div
                  class="absolute inset-0 flex items-center"
                  aria-hidden="true"
                >
                  <div class="h-0.5 w-full bg-gray-200" />
                </div>
                <span
                  class="group relative flex h-12 w-12 items-center justify-center rounded-full border-2 border-gray-300 bg-white border-gray-400"
                >
                  <component
                    :is="step.solidIcon"
                    class="h-8 w-8 text-gray-400"
                  ></component>
                  <span class="sr-only">{{ step.name }}</span>
                </span>
              </template>
            </li>
          </ol>
        </nav>
        <div
          class="flex items-center mt-6 justify-items-center grid grid-cols-4"
        >
          <router-view name="ChooseImage"></router-view>
          <router-view name="ConfigureImage"></router-view>
          <router-view name="StorageSelect"></router-view>
          <router-view name="FlashImage"></router-view>
        </div>
        <ProgressBar />
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { RouterView, useRouter } from "vue-router";
import {
  FolderArrowDownIcon as FolderArrowDownIconOutline,
  ServerIcon as ServerIconOutline,
  BoltIcon as BoltIconOutline,
  Cog6ToothIcon as CogToothOutline,
} from "@heroicons/vue/24/outline";
import {
  FolderArrowDownIcon as FolderArrowDownIconSolid,
  ServerIcon as ServerIconSolid,
  BoltIcon as BoltIconSolid,
  Cog6ToothIcon as CogToothSolid,
} from "@heroicons/vue/24/solid";
import ProgressBar from "@/components/steps/ProgressBar.vue";
import StickyAlerts from "@/components/alerts/StickyAlerts.vue";

const router = useRouter();
const steps = [
  {
    name: "Choose image",
    outlineIcon: FolderArrowDownIconOutline,
    solidIcon: FolderArrowDownIconSolid,
    status: () => {
      switch (router.currentRoute.value.name) {
        case "select-image":
          return "current";
        case "configure-image":
          return "complete";
        case "select-storage":
          return "complete";
        case "flash-image":
          return "complete";
      }
    },
  },
  {
    name: "Configue Image",
    outlineIcon: CogToothOutline,
    solidIcon: CogToothSolid,
    status: () => {
      switch (router.currentRoute.value.name) {
        case "select-image":
          return "upcoming";
        case "configure-image":
          return "current";
        case "select-storage":
          return "complete";
        case "flash-image":
          return "complete";
      }
    },
  },
  {
    name: "Choose disk",
    outlineIcon: ServerIconOutline,
    solidIcon: ServerIconSolid,
    status: () => {
      switch (router.currentRoute.value.name) {
        case "select-image":
          return "upcoming";
        case "configure-image":
          return "upcoming";
        case "select-storage":
          return "current";
        case "flash-image":
          return "complete";
      }
    },
  },
  {
    name: "Flash!",
    outlineIcon: BoltIconOutline,
    solidIcon: BoltIconSolid,
    status: () => {
      switch (router.currentRoute.value.name) {
        case "select-image":
          return "upcoming";
        case "configure-image":
          return "upcoming";
        case "select-storage":
          return "upcoming";
        case "flash-image":
          return "current";
      }
    },
  },
];
</script>

<template>
  <div v-if="!active" class="flex-1 h-20">
    <button
      disabled
      class="text-center block mx-4 my-4 h-12 w-48 block bg-indigo-400 text-white font-bold py-2 px-4 border-b-4 border-indigo-700 rounded disabled:opacity-50"
    >
      Configure Image
    </button>
  </div>
  <div v-else class="flex-1">
    <button
      class="text-center block mx-4 my-4 h-12 w-48 block bg-indigo-400 hover:bg-indigo-500 text-white font-bold py-2 px-4 border-b-4 border-indigo-700 hover:border-indigo-600 rounded"
      @click="onClick"
    >
      Configure Image
    </button>
    <TransitionRoot as="template" :show="formOpen">
      <Dialog as="div" class="relative z-10" @close="formOpen = false">
        <TransitionChild
          as="template"
          enter="ease-out duration-300"
          enter-from="opacity-0"
          enter-to="opacity-100"
          leave="ease-in duration-200"
          leave-from="opacity-100"
          leave-to="opacity-0"
        >
          <div
            class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity"
          />
        </TransitionChild>

        <div class="fixed inset-0 z-10 overflow-y-auto">
          <div
            class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0"
          >
            <TransitionChild
              as="template"
              enter="ease-out duration-300"
              enter-from="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
              enter-to="opacity-100 translate-y-0 sm:scale-100"
              leave="ease-in duration-200"
              leave-from="opacity-100 translate-y-0 sm:scale-100"
              leave-to="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
            >
              <DialogPanel
                class="relative transform overflow-hidden rounded-lg bg-white text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-4xl"
              >
                <div class="bg-white px-4 pt-5 pb-4 sm:p-6 sm:pb-4">
                  <div class="sm:flex sm:items-start bg-stone-100 p-2 rounded">
                    <div
                      class="mx-auto flex h-12 w-12 flex-shrink-0 items-center justify-center rounded-full bg-indigo-100 sm:mx-0 sm:h-10 sm:w-10"
                    >
                      <Cog6ToothIcon
                        class="h-6 w-6 text-indigo-600"
                        aria-hidden="true"
                      />
                    </div>
                    <div class="mt-3 text-center sm:mt-0 sm:ml-4 sm:text-left">
                      <DialogTitle
                        as="h3"
                        class="text-lg font-medium leading-6 text-gray-900"
                        >Customize Image</DialogTitle
                      >
                      <div class="mt-2">
                        <p class="text-sm text-gray-500">
                          Image customizations depend on
                          <a
                            href="https://cloud-init.io/"
                            class="text-indigo-500 hover:text-indigo-600"
                            target="_blank"
                            >cloud-init</a
                          >, which comes pre-installed in PrintNanny OS.
                        </p>
                      </div>
                    </div>
                  </div>
                  <!-- basic settings form -->
                  <basic-settings-form :on-cancel="onCancel" />
                </div>
              </DialogPanel>
            </TransitionChild>
          </div>
        </div>
      </Dialog>
    </TransitionRoot>
  </div>
</template>
<script setup lang="ts">
import { computed, ref } from "vue";
import { useRouter } from "vue-router";
import { useStore } from "@/store";
import {
  Dialog,
  DialogPanel,
  DialogTitle,
  TransitionChild,
  TransitionRoot,
} from "@headlessui/vue";
import { Cog6ToothIcon } from "@heroicons/vue/24/outline";
import BasicSettingsForm from "@/components/forms/BasicSettingsForm.vue";

const router = useRouter();
const store = useStore();

const formOpen = ref(false);
const key = "configure-image";
const active = computed(() => router.currentRoute.value.name == key);

function onCancel() {
  formOpen.value = false;
}

function onClick() {
  formOpen.value = true;
}
</script>

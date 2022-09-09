<template>
<div class="flex-1">
    <p class="text-stone-50 text-2xl font-medium text-center">Storage</p>
    <button @click="onClick" class="bg-stone-300 hover:bg-stone-200 text-stone-900 font-bold py-2 px-4 border-b-4 border-stone-500 hover:border-stone-300 rounded">{{ store.selectedDisk !== undefined ? `${store.selectedDisk?.vendor} ${store.selectedDisk?.model} (${store.selectedDisk?.size})` :'Choose Storage'}}</button>
    <TransitionRoot as="template" :show="show">
        <Dialog as="div" class="relative z-10" @close="show = false">
        <TransitionChild as="template" enter="ease-out duration-300" enter-from="opacity-0" enter-to="opacity-100" leave="ease-in duration-200" leave-from="opacity-100" leave-to="opacity-0">
            <div class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity" />
        </TransitionChild>
    
        <div class="fixed inset-0 z-10 overflow-y-auto">
            <div class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
            <TransitionChild as="template" enter="ease-out duration-300" enter-from="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95" enter-to="opacity-100 translate-y-0 sm:scale-100" leave="ease-in duration-200" leave-from="opacity-100 translate-y-0 sm:scale-100" leave-to="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95">
                <DialogPanel class="relative transform overflow-hidden rounded-lg bg-white text-left shadow-xl transition-all min-w-full-3/4 min-h-full">
                <div class="bg-white px-4 pt-5 pb-4 sm:p-6 sm:pb-4">
                    <div class="sm:flex sm:items-start">
                    <div class="mt-3 w-full text-center sm:mt-0 sm:ml-4 sm:text-left">
                        <DialogTitle as="h3" class="text-lg font-medium leading-6 text-gray-900">Select removeable USB storage:</DialogTitle>
                        <div class="mt-2">
                            <ul class="w-full text-sm font-medium text-gray-900 bg-white rounded-lg border border-gray-200 dark:bg-gray-700 dark:border-gray-600 dark:text-white">
                            <li class="w-full rounded-t-lg border-b border-gray-200 dark:border-gray-600" v-for="disk in store.removeableDisks" :key="disk.name">
                                <div class="flex items-center pl-3">
                                    <input id="list-radio-license" type="radio" v-model="store.selectedDisk"  :value="disk" name="list-radio" class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-700 focus:ring-2 dark:bg-gray-600 dark:border-gray-500">
                                    <label for="list-radio-license" class="p-4 ml-2 w-full text-sm font-medium text-gray-900 dark:text-gray-300"><strong>{{ disk.vendor }} {{ disk.model}} <span v-if="disk.partitions.length >0">({{ disk.partitions.map(p => p.label).join(', ')}})</span> - {{ disk.size}}</strong>
                                    <span v-if="disk.partitions.length >0"><br>Mounted as {{ disk.partitions.map(p => p.mountpoint).join(', ')}}</span>
                                    </label>
                                </div>
                            </li>
                        </ul>
                        </div>
                    </div>
                    </div>
                </div>
                <div class="bg-gray-50 px-4 py-3 sm:flex sm:flex-row-reverse sm:px-6">
                    <button type="button" class="inline-flex w-full justify-center rounded-md border border-transparent bg-indigo-600 px-4 py-2 text-base font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 sm:ml-3 sm:w-auto sm:text-sm" @click="show = false">Select</button>
                    <button type="button" class="mt-3 inline-flex w-full justify-center rounded-md border border-gray-300 bg-white px-4 py-2 text-base font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 sm:mt-0 sm:ml-3 sm:w-auto sm:text-sm" @click="show = false" ref="cancelButtonRef">Back</button>
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
import { ref } from 'vue'
import { Dialog, DialogPanel, DialogTitle, TransitionChild, TransitionRoot } from '@headlessui/vue'
import { ExclamationTriangleIcon } from '@heroicons/vue/24/outline'
import { useWizardStore } from '../../store/wizard';



const store = useWizardStore();
const show = ref(false);
const onClick = () => {
    console.log("clicked")
    show.value = true;
}

</script>
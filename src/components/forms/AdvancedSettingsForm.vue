<template>
    <div class="container m-auto">
      <Form class="space-y-8 divide-y divide-gray-200" @submit="onSubmit" v-slot="{ values }" :validation-schema="schema" :initial-values="store.savedCloudInit">
        <div class="space-y-8 divide-y divide-gray-200">
          <div class="pt-8">
            <div>
              <h3 class="text-lg font-medium leading-6 text-gray-900">Custom cloud-init / cloud-config data</h3>
              <p class="mt-1 text-sm text-gray-500">Complete customization control when you provide your own cloud-init configuration. <a href="https://cloudinit.readthedocs.io/en/latest/topics/examples.html" class="text-indigo-500 hover:text-indigo-600">See example cloud-config files.</a></p>
            </div>
            <div class="mt-6 grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
              <div class="sm:col-span-6">
                <label for="userData" class="block text-sm font-medium text-gray-700">User Data</label>
                <div class="mt-1">
                  <Field type="textarea" id="userData" name="userData" rows="12">
                    <textarea id="userData" name="userData" rows="12" :value="values.userData" class="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm" />
                  </Field>
                </div>
                <p class="mt-2 text-sm text-gray-500">User data should conform to cloud-init's <a href="https://cloudinit.readthedocs.io/en/latest/topics/format.html" class="text-indigo-500 text-indigo-600">User-Data</a> format.</p>
              </div>
            </div>
            <div class="mt-6 grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
              <div class="sm:col-span-6">
                <label for="networkData" class="block text-sm font-medium text-gray-700">Network Data</label>
                <div class="mt-1">
                  <Field type="textarea" id="networkData" name="networkData" rows="12">
                    <textarea id="networkData" name="networkData" rows="12" :value="values.networkData" class="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm" />
                  </Field>
                </div>
                <p class="mt-2 text-sm text-gray-500">Network data should conform to cloud-init's <a href="https://cloudinit.readthedocs.io/en/latest/topics/network-config-format-v2.html" class="text-indigo-500 text-indigo-600">Network Configuration Format</a> format.</p>
              </div>
            </div>
          </div>
        </div>
      </Form>
    </div>
</template>
<script setup lang="ts">
import { ref, computed } from "vue";
import * as yup from "yup";
import countryList from "@/utils/country";
import timezoneList from "@/utils/timezone";
import keyboardLayoutList from "@/utils/keyboard";
import { useWizardStore } from "@/store/wizard";
import { Form, Field, ErrorMessage, configure, useField, useForm} from 'vee-validate';
import keyboard from "../../utils/keyboard";

configure({
    validateOnBlur: true,
    validateOnChange: true,
    validateOnModelUpdate: true, 
})
const store = useWizardStore();
const props = defineProps({
    edition: {
    type: String,
    default: "octoprint"
    }
});


store.$patch({ edition: props.edition})

// define a validation schema
const schema = yup.object({
    userData: yup.string(),
    networkData: yup.string()
});

function onSubmit(values) {
    console.log("Form was submitted with values", values)
}
    
</script>
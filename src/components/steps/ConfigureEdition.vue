<!--
  This example requires Tailwind CSS v2.0+ 
  
  This example requires some changes to your config:
  
  ```
  // tailwind.config.js
  module.exports = {
    // ...
    plugins: [
      // ...
      require('@tailwindcss/forms'),
    ],
  }
  ```
-->
<template>
<div class="container m-auto">
  <Form class="space-y-8 divide-y divide-gray-200" @submit="onSubmit">
    <div class="space-y-8 divide-y divide-gray-200">
      <div>
        <div>
          <h3 class="text-lg font-medium leading-6 text-gray-900">Configuring {{ edition }}</h3>
          <p class="mt-1 text-sm text-gray-500">This information is used to configure your Raspberry Pi. Passwords and other sensitive fields are not shared with PrintNanny.</p>
        </div>

        <div class="mt-6 grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
          <div class="sm:col-span-4">
            <label for="hostname" class="block text-sm font-medium text-gray-700">Set hostname</label>
            <div class="mt-1 flex rounded-md shadow-sm">
              <Field type="text" name="hostname" id="hostname" class="block w-full min-w-0 flex-1 rounded-none rounded-l-md border-gray-300 focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm" />
              <span class="inline-flex items-center rounded-r-md border border-l-0 border-gray-300 bg-gray-50 px-3 text-gray-500 sm:text-sm">.local</span>

            </div>
            <ErrorMessage name="hostname" class="text-sm text-red-500" />

          </div>


          <div class="sm:col-span-6">
            <label for="about" class="block text-sm font-medium text-gray-700">About</label>
            <div class="mt-1">
              <textarea id="about" name="about" rows="3" class="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm" />
            </div>
            <p class="mt-2 text-sm text-gray-500">Write a few sentences about yourself.</p>
          </div>

          <div class="sm:col-span-6">
            <label for="photo" class="block text-sm font-medium text-gray-700">Photo</label>
            <div class="mt-1 flex items-center">
              <span class="h-12 w-12 overflow-hidden rounded-full bg-gray-100">
                <svg class="h-full w-full text-gray-300" fill="currentColor" viewBox="0 0 24 24">
                  <path d="M24 20.993V24H0v-2.996A14.977 14.977 0 0112.004 15c4.904 0 9.26 2.354 11.996 5.993zM16.002 8.999a4 4 0 11-8 0 4 4 0 018 0z" />
                </svg>
              </span>
              <button type="button" class="ml-5 rounded-md border border-gray-300 bg-white py-2 px-3 text-sm font-medium leading-4 text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">Change</button>
            </div>
          </div>

          <div class="sm:col-span-6">
            <label for="cover-photo" class="block text-sm font-medium text-gray-700">Cover photo</label>
            <div class="mt-1 flex justify-center rounded-md border-2 border-dashed border-gray-300 px-6 pt-5 pb-6">
              <div class="space-y-1 text-center">
                <svg class="mx-auto h-12 w-12 text-gray-400" stroke="currentColor" fill="none" viewBox="0 0 48 48" aria-hidden="true">
                  <path d="M28 8H12a4 4 0 00-4 4v20m32-12v8m0 0v8a4 4 0 01-4 4H12a4 4 0 01-4-4v-4m32-4l-3.172-3.172a4 4 0 00-5.656 0L28 28M8 32l9.172-9.172a4 4 0 015.656 0L28 28m0 0l4 4m4-24h8m-4-4v8m-12 4h.02" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
                </svg>
                <div class="flex text-sm text-gray-600">
                  <label for="file-upload" class="relative cursor-pointer rounded-md bg-white font-medium text-indigo-600 focus-within:outline-none focus-within:ring-2 focus-within:ring-indigo-500 focus-within:ring-offset-2 hover:text-indigo-500">
                    <span>Upload a file</span>
                    <input id="file-upload" name="file-upload" type="file" class="sr-only" />
                  </label>
                  <p class="pl-1">or drag and drop</p>
                </div>
                <p class="text-xs text-gray-500">PNG, JPG, GIF up to 10MB</p>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="pt-8">
        <div>
          <h3 class="text-lg font-medium leading-6 text-gray-900">Wireless LAN</h3>
          <p class="mt-1 text-sm text-gray-500">Configure wifi network information. Your network password is never shared with PrintNanny.</p>
        </div>
        <div class="mt-6 grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">

          <div class="sm:col-span-3">
            <div class="mt-1">
              <div class="sm:col-span-3">
                <label for="wifiSSID" class="block text-sm font-medium text-gray-700">Network SSID</label>
                <div class="mt-1">
                  <Field type="text" name="wifiSSID" id="wifiSSID" class="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm" />
                </div>
              </div>
            </div>
          </div>
          <div class="sm:col-span-3">
            <div class="mt-1">
              <div class="sm:col-span-3">
                <label for="wifiPassword" class="block text-sm font-medium text-gray-700">Network Password</label>
                <div class="mt-1">
                  <Field :type="wifiPasswordFieldType" name="wifiPassword" id="wifiPassword" class="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm" />
                </div>
                  <div class="relative flex items-start mt-1">
                    <div class="flex h-5 items-center">
                      <Field type="checkbox" name="showWifiPassword" id="showWifiPassword" :standalone="true" v-slot="{ field }" :value="false" class="h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-500">
                        <input type="checkbox" name="showWifiPassword" v-bind="field" :value="false" @change="() => onWifiPasswordShow(field.checked)" />

                      </Field>
                    </div>
                    <div class="ml-3 text-sm">
                      <label for="showWifiPassword" class="font-medium text-gray-700">Show wifi password</label>
                    </div>
                  </div>
 
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="pt-8">
        <div>
          <h3 class="text-lg font-medium leading-6 text-gray-900">Locale Settings</h3>
          <p class="mt-1 text-sm text-gray-500">Configure keyboard layout and timezone.</p>
        </div>
        <div class="mt-6 grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">

          <div class="sm:col-span-3">
            <label for="timezone" class="block text-sm font-medium text-gray-700">Set timezone</label>
            <div class="mt-1">
              <Field id="timezone"  v-slot="{ value }" name="timezone" as="select" class="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm">
                <option value="" disabled>Select timezone</option>
                <option v-for="timezone in timezoneList" :key="timezone" value="timezone" :selected="value && value.includes(timezone)">{{ timezone }}</option>
              </Field>
            </div>
          </div>
          <div class="sm:col-span-3">
            <label for="keyboardLayout" class="block text-sm font-medium text-gray-700">Set keyboard layout</label>
            <div class="mt-1">
              <Field id="keyboardLayout"  v-slot="{ value }" name="keyboardLayout" as="select" class="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm">
                <option value="" disabled>Select keyboard layout</option>
                <option v-for="item in keyboardLayoutList" :key="item" value="timezone" :selected="value && value.includes(iteme)">{{ item }}</option>
              </Field>
            </div>
          </div>
        </div>
      </div>


      <div class="pt-8">
        <div>
          <h3 class="text-lg font-medium leading-6 text-gray-900">Personal Information</h3>
          <p class="mt-1 text-sm text-gray-500">Use a permanent address where you can receive mail.</p>
        </div>
        <div class="mt-6 grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
          <div class="sm:col-span-3">
            <label for="first-name" class="block text-sm font-medium text-gray-700">First name</label>
            <div class="mt-1">
              <input type="text" name="first-name" id="first-name" autocomplete="given-name" class="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm" />
            </div>
          </div>

          <div class="sm:col-span-3">
            <label for="last-name" class="block text-sm font-medium text-gray-700">Last name</label>
            <div class="mt-1">
              <input type="text" name="last-name" id="last-name" autocomplete="family-name" class="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm" />
            </div>
          </div>

          <div class="sm:col-span-4">
            <label for="email" class="block text-sm font-medium text-gray-700">Email address</label>
            <div class="mt-1">
              <input id="email" name="email" type="email" autocomplete="email" class="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm" />
            </div>
          </div>

          <div class="sm:col-span-3">
            <label for="country" class="block text-sm font-medium text-gray-700">Country</label>
            <div class="mt-1">
              <select id="country" name="country" autocomplete="country-name" class="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm">
                <option>United States</option>
                <option>Canada</option>
                <option>Mexico</option>
              </select>
            </div>
          </div>

          <div class="sm:col-span-6">
            <label for="street-address" class="block text-sm font-medium text-gray-700">Street address</label>
            <div class="mt-1">
              <input type="text" name="street-address" id="street-address" autocomplete="street-address" class="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm" />
            </div>
          </div>

          <div class="sm:col-span-2">
            <label for="city" class="block text-sm font-medium text-gray-700">City</label>
            <div class="mt-1">
              <input type="text" name="city" id="city" autocomplete="address-level2" class="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm" />
            </div>
          </div>

          <div class="sm:col-span-2">
            <label for="region" class="block text-sm font-medium text-gray-700">State / Province</label>
            <div class="mt-1">
              <input type="text" name="region" id="region" autocomplete="address-level1" class="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm" />
            </div>
          </div>

          <div class="sm:col-span-2">
            <label for="postal-code" class="block text-sm font-medium text-gray-700">ZIP / Postal code</label>
            <div class="mt-1">
              <input type="text" name="postal-code" id="postal-code" autocomplete="postal-code" class="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm" />
            </div>
          </div>
        </div>
      </div>

      <div class="pt-8">
        <div>
          <h3 class="text-lg font-medium leading-6 text-gray-900">Notifications</h3>
          <p class="mt-1 text-sm text-gray-500">We'll always let you know about important changes, but you pick what else you want to hear about.</p>
        </div>
        <div class="mt-6">
          <fieldset>
            <legend class="sr-only">By Email</legend>
            <div class="text-base font-medium text-gray-900" aria-hidden="true">By Email</div>
            <div class="mt-4 space-y-4">
              <div class="relative flex items-start">
                <div class="flex h-5 items-center">
                  <input id="comments" name="comments" type="checkbox" class="h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-500" />
                </div>
                <div class="ml-3 text-sm">
                  <label for="comments" class="font-medium text-gray-700">Comments</label>
                  <p class="text-gray-500">Get notified when someones posts a comment on a posting.</p>
                </div>
              </div>
              <div class="relative flex items-start">
                <div class="flex h-5 items-center">
                  <input id="candidates" name="candidates" type="checkbox" class="h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-500" />
                </div>
                <div class="ml-3 text-sm">
                  <label for="candidates" class="font-medium text-gray-700">Candidates</label>
                  <p class="text-gray-500">Get notified when a candidate applies for a job.</p>
                </div>
              </div>
              <div class="relative flex items-start">
                <div class="flex h-5 items-center">
                  <input id="offers" name="offers" type="checkbox" class="h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-500" />
                </div>
                <div class="ml-3 text-sm">
                  <label for="offers" class="font-medium text-gray-700">Offers</label>
                  <p class="text-gray-500">Get notified when a candidate accepts or rejects an offer.</p>
                </div>
              </div>
            </div>
          </fieldset>
          <fieldset class="mt-6">
            <legend class="contents text-base font-medium text-gray-900">Push Notifications</legend>
            <p class="text-sm text-gray-500">These are delivered via SMS to your mobile phone.</p>
            <div class="mt-4 space-y-4">
              <div class="flex items-center">
                <input id="push-everything" name="push-notifications" type="radio" class="h-4 w-4 border-gray-300 text-indigo-600 focus:ring-indigo-500" />
                <label for="push-everything" class="ml-3 block text-sm font-medium text-gray-700">Everything</label>
              </div>
              <div class="flex items-center">
                <input id="push-email" name="push-notifications" type="radio" class="h-4 w-4 border-gray-300 text-indigo-600 focus:ring-indigo-500" />
                <label for="push-email" class="ml-3 block text-sm font-medium text-gray-700">Same as email</label>
              </div>
              <div class="flex items-center">
                <input id="push-nothing" name="push-notifications" type="radio" class="h-4 w-4 border-gray-300 text-indigo-600 focus:ring-indigo-500" />
                <label for="push-nothing" class="ml-3 block text-sm font-medium text-gray-700">No push notifications</label>
              </div>
            </div>
          </fieldset>
        </div>
      </div>
    </div>

    <div class="pt-5">
      <div class="flex justify-end">
        <button type="button" class="rounded-md border border-gray-300 bg-white py-2 px-4 text-sm font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">Cancel</button>
        <button type="submit" class="ml-3 inline-flex justify-center rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">Save</button>
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
})

const showWifiPassword = ref(false)

const wifiPasswordFieldType = ref("password");

function onWifiPasswordShow(value: boolean){
  if (value == true){
    wifiPasswordFieldType.value = "text"
  } else {
    wifiPasswordFieldType.value ="password"
  }
}


store.$patch({ edition: props.edition})

// define a validation schema
const schema = yup.object({
  email: yup.string().required().email(),
  hostname: yup.string().required().matches(/^[a-z0-9]+$/i),
  enableSSH: yup.bool().default(true),
  enableBasicAuth:  yup.bool().default(true),
  username: yup.string().required().matches(/^[a-z0-9]+$/i),
  password: yup.string().required(),
  enableSSHKeyAuth: yup.bool(),
  sshAuthorizedKeys: yup.string(),
  enableWifi: yup.bool(),
  wifiSSID: yup.string(),
  wifiPassword: yup.string(),
  wifiCountry: yup.string().oneOf(countryList),
  timezone: yup.string().oneOf(timezoneList),
  keyboardLayout: yup.string().oneOf(keyboardLayoutList),
  enableTelemetry: yup.bool(),
});


const { handleSubmit } = useForm({ validationSchema: schema});

const { checked, } = useField('showWifiPassword', undefined, {
  type: 'checkbox',
  checkedValue: true,
  uncheckedValue: false,
});


const onSubmit = handleSubmit(values => {
  console.log("Form was submitted with values", values)
})

</script>
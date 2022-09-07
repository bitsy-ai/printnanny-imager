<template>
    
</template>
<script setup lang="ts">
import * as yup from "yup";
import countryList from "@/utils/country";
import timezoneList from "@/utils/timezone";
import keyboardLayoutList from "@/utils/keyboard";

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
  timeZone: yup.string().oneOf(timezoneList),
  keyboardLayout: yup.string().oneOf(keyboardLayoutList),
  enableTelemetry: yup.bool(),
});

</script>
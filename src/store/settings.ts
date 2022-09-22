import { defineStore, acceptHMRUpdate } from "pinia";
import type { CloudInitForm } from "@/utils/cloudInit";
import { CloudInitGenerator } from "@/utils/cloudInit";
export const useSettingsStore = defineStore({
  id: "settings",
  persist: true,
  state: () => ({
    savedFormValues: null as null | CloudInitForm,
  }),
  actions: {
    saveForm(fieldset: CloudInitForm): CloudInitForm {
      // encrypt sensitive fields
      const encryptedFieldset = CloudInitGenerator.encryptSensitive(fieldset);
      this.$patch({ savedFormValues: encryptedFieldset });
      return encryptedFieldset;
    },
  },
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useSettingsStore, import.meta.hot));
}

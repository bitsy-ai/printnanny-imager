import { defineStore, acceptHMRUpdate } from "pinia";
export const useSettingsStore = defineStore({
  id: "settings",
  persist: true,
  state: () => ({
    savedFormValues: undefined as undefined | any,
  }),
  actions: {},
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useSettingsStore, import.meta.hot));
}

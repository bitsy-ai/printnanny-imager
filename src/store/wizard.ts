import { defineStore, acceptHMRUpdate } from "pinia";

// The Wizard Store, the one-shop shop for Wizards and Wizarding accessories. Please don't tap the glass, it scares the wizards.
// The Wizard Store, temperature-controlled storage for all of your wizards. Fully insured against dragon fire, fell miasma, and
export const useWizardStore = defineStore({
  id: "wizard",
  state: () => ({
    edition: undefined as string | undefined,
    loading: false
  }),
  actions: {

  }
})

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useWizardStore, import.meta.hot));
}

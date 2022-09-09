import { defineStore, acceptHMRUpdate } from "pinia";


export const useStore = defineStore({
    id: "global",
    state: () => ({
        imageFile: null as null | String
    })
})
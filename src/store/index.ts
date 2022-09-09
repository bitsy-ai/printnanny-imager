import { defineStore, acceptHMRUpdate } from "pinia";
import { RemoveableDisk } from "@/types";
import { listRemoveableDisks } from "../utils/disk";


export const useStore = defineStore({
    id: "global",
    state: () => ({
        selectedImageFile: null as null | String,
        selectedDisk: null as null | RemoveableDisk,
        removeableDisks: [] as Array<RemoveableDisk>,
        loading: false
    }),
    actions: {
        async listRemoveableDrives(): Promise<Array<RemoveableDisk>> {
            this.$patch({ loading: true });
            let disks = await listRemoveableDisks();
            this.$patch({ removeableDisks: disks, loading: false })
        }
    },
})
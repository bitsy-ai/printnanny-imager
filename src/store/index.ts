import { defineStore, acceptHMRUpdate } from "pinia";
import { RemoveableCrossPlatformDisk } from "@/types";
import { listRemoveableDisks } from "../utils/disk";

export const useStore = defineStore({
  id: "global",
  state: () => ({
    selectedImageFile: null as null | string,
    selectedDisk: null as null | RemoveableCrossPlatformDisk,
    removeableDisks: [] as Array<RemoveableCrossPlatformDisk>,
    loading: false,
  }),
  actions: {
    async listRemoveableDrives(): Promise<Array<RemoveableDisk>> {
      this.$patch({ loading: true });
      const disks = await listRemoveableDisks();
      this.$patch({ removeableDisks: disks, loading: false });
      return disks;
    },
  },
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useStore, import.meta.hot));
}

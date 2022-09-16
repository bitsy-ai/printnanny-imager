import { defineStore, acceptHMRUpdate } from "pinia";
import { CrossPlatformDisk, ImageWriteProgress } from "@/types";
import { listRemoveableDisks } from "../utils/disk";

export const useStore = defineStore({
  id: "global",
  state: () => ({
    selectedImageFile: null as null | string,
    selectedDisk: null as null | CrossPlatformDisk,
    removeableDisks: [] as Array<CrossPlatformDisk>,
    loading: false,
    progress: null as null | ImageWriteProgress,
  }),
  actions: {
    async listRemoveableDrives(): Promise<Array<CrossPlatformDisk>> {
      this.$patch({ loading: true });
      const disks = await listRemoveableDisks();
      console.log("Found removeable disks", disks);

      this.$patch({ removeableDisks: disks, loading: false });
      return disks;
    },
  },
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useStore, import.meta.hot));
}

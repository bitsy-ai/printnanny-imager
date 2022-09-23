import { defineStore, acceptHMRUpdate } from "pinia";
import { CrossPlatformDisk, ImageWriteProgress } from "@/types";
import { listRemoveableDisks } from "../utils/disk";
import type { UiAlert } from "@/types";

export const useStore = defineStore({
  id: "global",
  state: () => ({
    alerts: [] as Array<UiAlert>,
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
    showError(uialert: UiAlert) {
      // show at most 1 alert message with the same body
      const alreadyShown = this.alerts.filter(
        (a) => a.header == uialert.header
      );
      if (alreadyShown.length === 0) {
        this.alerts.push(uialert);
      }
    },
  },
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useStore, import.meta.hot));
}

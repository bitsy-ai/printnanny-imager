import { defineStore, acceptHMRUpdate } from "pinia";
import { invoke } from '@tauri-apps/api/tauri';
import type { OperatingSystem, RemoveableDisk, SingleBoardComputer } from "../types";
import boards from "@/data/boards";
import operatingSytems from "@/data/os";
import { Command } from '@tauri-apps/api/shell'
import { platform } from '@tauri-apps/api/os';
import { listRemoveableDisks } from "../utils/disk";
import { threadId } from "worker_threads";

// The Wizard Store, the one-shop shop for Wizards and Wizarding accessories. Please don't tap the glass, it scares the wizards.
// The Wizard Store, temperature-controlled storage for all of your wizards. Fully insured against dragon fire, fell miasma, and
export const useWizardStore = defineStore({
  id: "wizard",
  state: () => ({
    board: Object.values(boards)[0] as SingleBoardComputer,
    os: Object.values(operatingSytems)[0] as OperatingSystem,
    loadingDisks: false,
    removeableDisks: [] as Array<RemoveableDisk>,
    selectedDisk: undefined as undefined | RemoveableDisk,
    edition: undefined as string | undefined,
    loading: false,
    savedFormValues: {
      hostname: "printnanny",
      username: "pi",
    },
    savedCloudInit: {
      userData: `#cloud-config
manage_etc_hosts: true
users:
- default
- name: pi
  groups: users,adm,dialout,audio,netdev,video,plugdev,cdrom,games,input,gpio,spi,i2c,render,sudo
  shell: /bin/bash
  lock_passwd: true

timezone: America/Los_Angeles
`,
      networkData: `version:2
wifis:
renderer: networkd
# custom configuration goes here
`,
    },
  }),
  actions: {

    async listRemoveableDrives(): Promise<Array<RemoveableDisk>> {
      this.$patch({ loadingDisks: true });
      let disks = await listRemoveableDisks();
      this.$patch({ removeableDisks: disks, loadingDisks: false })
    }
  },
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useWizardStore, import.meta.hot));
}

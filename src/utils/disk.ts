import { Child, Command } from "@tauri-apps/api/shell";
import { platform } from "@tauri-apps/api/os";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";

import {
  CrossPlatformDisk,
  ImageWriteProgress,
  ImageWriteProgressInterface,
} from "../types";
import { useStore } from "@/store";
import { useSettingsStore } from "@/store/settings";
import { CloudInitGenerator } from "./cloudInit";
import { showError } from "./error";

async function listRemoveableDisks(): Promise<Array<CrossPlatformDisk>> {
  const output = await invoke("list_diskdrive_crossplatform");
  if (output) {
    const parsed = JSON.parse(output as string);
    return parsed.map((d: any) => new CrossPlatformDisk(d));
  }
  return [];
}

async function writeImage(disk: CrossPlatformDisk, imagePath: string) {
  // listen for image progress events
  const store = useStore();
  const unlisten = await listen<string>("image_write_progress", (event) => {
    // console.log(`Got image_write_progress, payload:`, event);
    const payload = event.payload as unknown;
    const parsed = payload as ImageWriteProgressInterface;
    const progress = new ImageWriteProgress(parsed);
    store.$patch({ progress: progress });
  });

  console.log("Created listener");

  await invoke("write_image", {
    imagePath: imagePath,
    diskPath: disk.path,
    deviceId: disk.deviceId,
  }).catch(showError);
  console.log(`Finished writing ${imagePath} to ${disk.path}`);

  // clean up listener
  unlisten();
}

// write /boot/user-data and /boot/network-config
async function writeBootfiles(disk: CrossPlatformDisk) {
  console.log("Writing bootfiles to disk", disk);
  const store = useStore();
  const settingsStore = useSettingsStore();
  const progress = new ImageWriteProgress({
    label: "Finalizing image...",
    percent: 25,
  });
  store.$patch({ progress: progress });
  if (settingsStore.savedFormValues) {
    const generator = new CloudInitGenerator(settingsStore.savedFormValues);
    let filename = CloudInitGenerator.userDataFilename();
    await invoke("write_bootfile", {
      diskPath: disk.path,
      filename: filename,
      contents: generator.generateUserData(),
    }).catch(showError);
    console.log(`Success! Wrote ${filename}`);
    let progress = new ImageWriteProgress({
      label: "Finalizing image...",
      percent: 75,
    });
    store.$patch({ progress: progress });
    filename = CloudInitGenerator.networkDataFilename();
    await invoke("write_bootfile", {
      diskPath: disk.path,
      filename: filename,
      contents: generator.generateNetworkData(),
    }).catch(showError);
    console.log(`Success! Wrote ${filename}`);
    progress = new ImageWriteProgress({
      label: "Finalizing image...",
      percent: 100,
    });
    store.$patch({ progress: progress });
  } else {
    const progress = new ImageWriteProgress({
      label: "Finalizing image...",
      percent: 100,
    });
    store.$patch({ progress: progress });
  }
}

async function flashImage(disk: CrossPlatformDisk, imagePath: string) {
  const platformName = await platform();
  let command = null as null | Command;
  let child = null as null | Child;
  switch (platformName) {
    case "linux":
      command = await new Command("write-image--linux", [
        "dd",
        "bs=4M",
        "status=progress",
        `if=${imagePath}`,
        `of=${disk.path}`,
      ]);
      command.on("close", (data) => {
        console.log(
          `command finished with code ${data.code} and signal ${data.signal}`
        );
      });
      command.on("error", (error) =>
        console.error(`command error: "${error}"`)
      );
      command.stdout.on("data", (line) =>
        console.log(`command stdout: "${line}"`)
      );
      command.stderr.on("data", (line) =>
        console.log(`command stderr: "${line}"`)
      );
      child = await command.spawn();
      console.log("flashImage pid: ", child.pid);
      break;
    case "win32":
    case "darwin":
      await writeImage(disk, imagePath);
      break;
  }
}

export { listRemoveableDisks, flashImage, writeBootfiles };

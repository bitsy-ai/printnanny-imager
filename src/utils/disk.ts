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

async function listRemoveableDisks(): Promise<Array<CrossPlatformDisk>> {
  const output = await invoke("list_diskdrive_crossplatform");
  if (output) {
    const parsed = JSON.parse(output as string);
    return parsed.map((d: any) => new CrossPlatformDisk(d));
  }
  return [];
}

async function writeImageDarwin(disk: CrossPlatformDisk, imagePath: string) {
  // listen for image progress events
  const store = useStore();
  const unlisten = await listen<string>("image_write_progress", (event) => {
    // console.log(`Got image_write_progress, payload:`, event);
    const parsed = event.payload as ImageWriteProgressInterface;
    const progress = new ImageWriteProgress(parsed);
    store.$patch({ progress: progress });
  });

  console.log("Created listener");

  await invoke("write_image_darwin", { imagePath: imagePath, disk: disk.path });
  console.log(`Finished writing ${imagePath} to ${disk.path}`);

  // clean up listener
  unlisten();
}

async function flashImage(disk: CrossPlatformDisk, imagePath: string) {
  const platformName = await platform();
  let command = null as null | Command;
  let child = null as null | Child;
  switch (platformName) {
    case "darwin":
      await writeImageDarwin(disk, imagePath);
      break;
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
      break;
  }
}

export { listRemoveableDisks, flashImage };

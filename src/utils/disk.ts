import { Child, ChildProcess, Command } from "@tauri-apps/api/shell";
import { platform } from "@tauri-apps/api/os";
import { RemoveableDisk, RemoveableDiskPartition } from "../types";

function parseLinuxDisks(jsonStr: string): Array<RemoveableDisk> {
  const parsed = JSON.parse(jsonStr);
  console.log(parsed);
  if (parsed.blockdevices == undefined) {
    console.error("Failed to parse lsblk output", parsed);
    return [];
  }
  const usbDevices = parsed.blockdevices.filter(
    (device: any) => device.tran == "usb"
  );
  if (usbDevices.length == 0) {
    return [];
  }
  return usbDevices.map((device: any) => {
    const partitions =
      device.children && device.children.length > 0
        ? device.children.map((part: any) => {
            return {
              name: part.name,
              label: part.label,
              size: part.size,
              mountpoint: part.mountpoint,
            } as RemoveableDiskPartition;
          })
        : [];

    return {
      key: device.name,
      model: device.model,
      name: device.name,
      path: device.path,
      partitions: partitions,
      serial: device.serial,
      size: device.size,
      vendor: device.vendor,
    } as RemoveableDisk;
  });
}

async function listRemoveableDisks(): Promise<Array<RemoveableDisk>> {
  const platformName = await platform();
  let output = null as null | ChildProcess;
  switch (platformName) {
    case "darwin":
      output = await new Command("list-diskdrive--macOS").execute();
      console.log(JSON.parse(output.stdout));
      return [];
      break;
    case "linux":
      output = await new Command("list-diskdrive--linux").execute();
      return parseLinuxDisks(output.stdout);
      break;
    case "win32":
      output = await new Command("list-diskdrive--windows").execute();
      console.log(JSON.parse(output.stdout));
      return [];
      break;
    default:
      console.error(
        `${platformName} is not supported. Please open a Github issue to request support.`
      );
      return [];
  }
}

async function flashImage(disk: RemoveableDisk, imagePath: string) {
  const platformName = await platform();
  let command = null as null | Command;
  let child = null as null | Child;
  switch (platformName) {
    case "darwin":
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

export { listRemoveableDisks, parseLinuxDisks, flashImage };

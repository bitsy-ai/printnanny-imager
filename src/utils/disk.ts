import { Command } from '@tauri-apps/api/shell'
import { platform } from '@tauri-apps/api/os';
import { RemoveableDisk, RemoveableDiskPartition } from '../types';

function parseLinuxDisks(jsonStr: string): Array<RemoveableDisk> {
  const parsed = JSON.parse(jsonStr);
  console.log(parsed)
  if (parsed.blockdevices == undefined) { console.error("Failed to parse lsblk output", parsed); return [] }
  const usbDevices = parsed.blockdevices.filter((device: any) => device.tran == "usb");
  if (usbDevices.length == 0) { return [] }
  return usbDevices.map((device: any) => {
    const partitions = device.children && device.children.length > 0 ? device.children.map((part: any) => {
      return {
        name: part.name,
        label: part.label,
        size: part.size,
        mountpoint: part.mountpoint,
      } as RemoveableDiskPartition
    }) : [];

    return {
      key: device.name,
      name: device.name,
      serial: device.serial,
      model: device.model,
      vendor: device.vendor,
      partitions: partitions,
      size: device.size
    } as RemoveableDisk
  })
}

async function listRemoveableDisks(): Promise<Array<RemoveableDisk>> {
  const platformName = await platform();
  switch (platformName) {
    case 'darwin':
      const macOSOutput = await new Command('list-diskdrive--macOS').execute();
      console.log(JSON.parse(macOSOutput.stdout));
      return []
      break
    case 'linux':
      const linuxOutput = await new Command('list-diskdrive--linux').execute();
      return parseLinuxDisks(linuxOutput.stdout)
    case 'win32':
      const output = await new Command('list-diskdrive--windows').execute();
      console.log(JSON.parse(output.stdout));
      return []
      break
    default:
      console.error(`${platformName} is not supported. Please open a Github issue to request support.`)
      return []
  }
}

export {
  listRemoveableDisks,
  parseLinuxDisks
}
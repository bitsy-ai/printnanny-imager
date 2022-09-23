import { stringify } from "yaml";
import { invoke } from "@tauri-apps/api/tauri";
import { showError } from "@/utils/error";

interface CloudInitForm {
  disableSSHPassword: boolean;
  enableRemoteVideo: boolean;
  enableSSH: boolean;
  enableTelemetry: boolean;
  hostname: string;
  keyboardLayout: string;
  password: string;
  saveSettings: boolean;
  timezone: string;
  username: string;
  wifiCountry: string;
  wifiPassword: string;
  wifiSSID: string;
  wifiSSIDHidden: boolean;
  sshAuthorizedKeys?: string;
}

class CloudInitGenerator implements CloudInitForm {
  disableSSHPassword: boolean;
  enableRemoteVideo: boolean;
  enableSSH: boolean;
  enableTelemetry: boolean;
  hostname: string;
  keyboardLayout: string;
  password: string;
  saveSettings: boolean;
  timezone: string;
  username: string;
  wifiCountry: string;
  wifiPassword: string;
  wifiSSID: string;
  wifiSSIDHidden: boolean;
  sshAuthorizedKeys?: string;

  constructor(args: CloudInitForm) {
    this.sshAuthorizedKeys = args.sshAuthorizedKeys;
    this.disableSSHPassword = args.disableSSHPassword;
    this.enableRemoteVideo = args.enableRemoteVideo;
    this.enableSSH = args.enableSSH;
    this.enableTelemetry = args.enableTelemetry;
    this.hostname = args.hostname;
    this.keyboardLayout = args.keyboardLayout;
    this.password = args.password;
    this.saveSettings = args.saveSettings;
    this.timezone = args.timezone;
    this.username = args.username;
    this.wifiCountry = args.wifiCountry;
    this.wifiPassword = args.wifiPassword;
    this.wifiSSID = args.wifiSSID;
    this.wifiSSIDHidden = args.wifiSSIDHidden;
  }

  static async encryptSensitive(args: CloudInitForm): Promise<CloudInitForm> {
    // perform a deep copy
    const result = JSON.parse(JSON.stringify(args));

    if (result.password !== undefined) {
      result.password = await CloudInitGenerator.hashPassword(
        result.password
      ).catch(showError);
    }
    if (result.wifiPassword !== undefined) {
      result.wifiPassword = await CloudInitGenerator.hashPassword(
        result.wifiPassword
      ).catch(showError);
    }
    return result;
  }

  static async comparePassword(
    password: string,
    hash: string
  ): Promise<boolean> {
    return await invoke("compare_password", { password, hash });
  }

  static async hashPassword(password: string): Promise<string> {
    return await invoke("hash_password", { password: password });
  }

  static defaultGroups(): string[] {
    return [
      "adm",
      "audio",
      "cdrom",
      "games",
      "gpio",
      "i2c",
      "input",
      "netdev",
      "plugdev",
      "render",
      "spi",
      "sudo",
      "users",
      "video",
    ];
  }

  static defaultShell(): string {
    return "/bin/bash";
  }

  static networkDataFilename(): string {
    return "network-config";
  }

  static metaDataFilename(): string {
    return "meta-data";
  }

  static vendorDataFilename(): string {
    return "vendor-data";
  }

  static userDataFilename(): string {
    return "user-data";
  }

  generateUserBlock() {
    const userBlock = {
      name: this.username,
      groups: CloudInitGenerator.defaultGroups(),
      lock_passwd: false,
      passwd: this.password, // password is already hashed before class is constructed
      shell: CloudInitGenerator.defaultShell(),
      sudo: "ALL=(ALL) NOPASSWD:ALL",
      ssh_authorized_keys: [] as string[],
    };
    if (this.sshAuthorizedKeys !== undefined) {
      userBlock.ssh_authorized_keys = this.sshAuthorizedKeys.split("\n");
    }
    return userBlock;
  }

  // not used currently, but cloud-init expects /boot/vendor-data to be present
  generateVendorData(): string {
    return "#cloud-config";
  }

  // not used currently, but cloud-init expects /boot/meta-data to be present
  generateMetaData(): string {
    return "";
  }
  // generate /boot/user-data contents
  generateUserData(): string {
    // create json data structure
    const jsonData = {
      hostname: this.hostname,
      manage_etc_hosts: true,
      users: ["default", this.generateUserBlock()],
      timezone: this.timezone,
    };
    // convert to yaml
    const yamlData = stringify(jsonData);
    // add #cloud-config to first line of generated yaml
    return `#cloud-config
${yamlData}
`;
  }

  // generate /boot/network-config contents
  generateNetworkData(): string {
    const jsonData = {
      version: 2,
      renderer: "networkd",
      wlan0: {
        dhcp4: true,
        optional: true,
        "access-points": { [this.wifiSSID]: { password: this.wifiPassword } },
      },
    };
    const yamlData = stringify(jsonData);
    return yamlData;
  }
}

export { CloudInitGenerator };
export type { CloudInitForm };

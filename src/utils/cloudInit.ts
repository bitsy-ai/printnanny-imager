import * as YAML from 'yaml';
import bcrypt from "bcrypt";

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

interface PasswordHashOptions {
    algorithm: string;
    saltLength: string;
    iterations: number;
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

  static comparePassword(password: string, hash: string): boolean {
    return bcrypt.compareSync(password, hash);
  }

  static hashPassword(password: string, saltRounds = 4096): string {
    return bcrypt.hashSync(password, saltRounds);
  }

  static defaultGroups() {
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

  static defaultShell() {
    return "/bin/bash";
  }

  generateUserBlock() {
    const userBlock = {
      name: this.username,
      groups: CloudInitGenerator.defaultGroups(),
      lock_passwd: false,
      passwd: this.password, // password is already hashed before class is constructed
      shell: CloudInitGenerator.defaultShell(),
      sudo: 'ALL=(ALL) NOPASSWD:ALL',
      ssh_authorized_keys: [] as string[]
    };
    if (this.sshAuthorizedKeys !== undefined){
        userBlock.ssh_authorized_keys=  this.sshAuthorizedKeys.split('\n');
    }
    return userBlock
  }

  generateUserData() {
    // create json data structure
    const jsonData = { hostname: this.hostname, manage_etc_hosts: true, users: [ "default", this.generateUserBlock()], timezone: this.timezone };
    // convert to yaml
    const yamlData = YAML.stringify(jsonData);
    // add #cloud-config to first line of generated yaml
    return `#cloud-config
${yamlData}
`
  }
}

export { CloudInitGenerator };
export type { CloudInitForm };

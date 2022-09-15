type SingleBoardComputer = {
  enabled: boolean;
  key: string;
  name: string;
  selected: boolean;
};

type OperatingSystem = {
  arch: string;
  key: string;
  name: string;
  release_index_url: string;
};

interface CrossPlatformDiskInterface {
  bootable: boolean;
  busProtocol: string;
  displayName: string;
  deviceId: string;
  path: string;
  size: string;
  sizePretty: string;
  isRemoveable: string;
  partitions: Array<CrossPlatformDisk>;
  volumeName: string;

  displayHeader(): string;
  displayDetail(): string;
}

class CrossPlatformDisk implements CrossPlatformDiskInterface {
  bootable: boolean;
  busProtocol: string;
  displayName: string;
  deviceId: string;
  path: string;
  size: string;
  sizePretty: string;
  isRemoveable: string;
  partitions: Array<CrossPlatformDisk>;
  volumeName: string;
  constructor(args: CrossPlatformDiskInterface) {
    this.bootable = args.bootable;
    this.busProtocol = args.busProtocol;
    this.displayName = args.displayName;
    this.deviceId = args.deviceId;
    this.path = args.path;
    this.size = args.size;
    this.sizePretty = args.sizePretty;
    this.volumeName = args.volumeName;
    this.isRemoveable = args.isRemoveable;
    this.partitions = args.partitions;
  }

  displayHeader(): string {
    return `${this.displayName} ${this.path} - ${this.sizePretty}`;
  }
  // TODO
  displayDetail(): string {
    return "";
    // return this.partitions.filter((p) => p.volumeName !== '').map(p => p.volumeName).join(', ')
  }
}

export type { SingleBoardComputer, OperatingSystem };

export { CrossPlatformDisk };

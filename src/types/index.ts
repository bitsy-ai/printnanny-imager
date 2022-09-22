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

interface ImageWriteProgressInterface {
  percent?: number;
  bytes_written?: number;
  bytes_total?: number;
  elapsed?: number;
  label: string;
}

class ImageWriteProgress {
  percent?: number;
  bytes_written?: number;
  bytes_total?: number;
  elapsed?: number;
  label: string;

  constructor(args: ImageWriteProgressInterface) {
    this.bytes_total = args.bytes_total;
    this.bytes_written = args.bytes_written;
    this.percent = args.percent;
    this.label = args.label;
    this.elapsed = args.elapsed;
  }

  display_percent(): string {
    if (this.percent !== undefined) {
      return `${this.percent}%`;
    } else if (
      this.bytes_total !== undefined &&
      this.bytes_written !== undefined
    ) {
      const perc = (this.bytes_written / this.bytes_total) * 100;
      console.log(perc);
      return `${Math.round(perc)}%`;
    }
    return "";
  }
}

export type {
  SingleBoardComputer,
  OperatingSystem,
  ImageWriteProgressInterface,
};

export { CrossPlatformDisk, ImageWriteProgress };

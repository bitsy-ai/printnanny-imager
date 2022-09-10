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

type RemoveableDiskPartition = {
  label: string;
  mountpoint: string;
  name: string;
  size: string;
};

type RemoveableLinuxDisk = {
  key: string;
  model: string;
  name: string;
  partitions: Array<RemoveableDiskPartition>;
  path: string;
  serial: string;
  size: string;
  vendor: string;
};

type RemoveableCrossPlatformDisk = {
  type: string;
  name: string;
  fileSystem: string;
  mountPoint: string;
  totalSpace: number;
  totalSpacePretty: string;
  availableSpace: number;
  availableSpacePretty: string;
  isRemoveable: boolean;
};

export type {
  SingleBoardComputer,
  OperatingSystem,
  RemoveableLinuxDisk,
  RemoveableDiskPartition,
  RemoveableCrossPlatformDisk,
};

type SingleBoardComputer = {
  enabled: Boolean;
  key: String;
  name: String;
  selected: Boolean;
}

type OperatingSystem = {
  arch: String;
  key: String;
  name: String;
  release_index_url: String;
}

type RemoveableDiskPartition = {
  label: String;
  mountpoint: String;
  name: String;
  size: String;
}

type RemoveableDisk = {
  key: String,
  model: String;
  name: String;
  partitions: Array<RemoveableDiskPartition>;
  path: String;
  serial: String;
  size: String;
  vendor: String;
}

export {
  SingleBoardComputer,
  OperatingSystem,
  RemoveableDisk,
  RemoveableDiskPartition,
}


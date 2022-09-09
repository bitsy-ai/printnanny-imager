type SingleBoardComputer = {
  key: String;
  name: String;
  enabled: Boolean;
  selected: Boolean;
}

type OperatingSystem = {
  name: String;
  key: String;
  release_index_url: String;
  arch: String;
}

type RemoveableDiskPartition = {
  mountpoint: String;
  name: String;
  label: String;
  size: String;
}

type RemoveableDisk = {
  key: String,
  name: String;
  serial: String;
  model: String;
  vendor: String;
  partitions: Array<RemoveableDiskPartition>,
  size: String
}

export {
  SingleBoardComputer,
  OperatingSystem,
  RemoveableDisk,
  RemoveableDiskPartition,
}


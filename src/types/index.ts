type SingleBoardComputer {
  key: String;
  name: String;
  enabled: Boolean;
  selected: Boolean;
}

type OperatingSystem {
  name: String;
  key: String;
  release_index_url: String;
  arch: String;
}

export {
  SingleBoardComputer,
  OperatingSystem
}
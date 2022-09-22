/** @type {import('ts-jest').JestConfigWithTsJest} */
module.exports = {
  // [...]
  preset: "ts-jest/presets/js-with-ts-esm",
  extensionsToTreatAsEsm: ['.ts'],
  testEnvironment: "jsdom",
  moduleNameMapper: {
    '^(\\.{1,2}/.*)\\.js$': '$1',
  },
  transform: {
    "^.+\\.m?[tj]sx?$": ['ts-jest', {
      useESM: true,
    },]
  }
}
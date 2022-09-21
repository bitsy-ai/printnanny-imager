import { describe, expect, test } from "@jest/globals";

import { CloudInitGenerator } from "../src/utils/cloudInit";
import type { CloudInitForm } from "../src/utils/cloudInit";

describe("CloudInitGenerator", () => {
  const baseFormData = {
    disableSSHPassword: false,
    enableRemoteVideo: true,
    enableSSH: true,
    enableTelemetry: true,
    hostname: "pn-test",
    keyboardLayout: "us",
    password: "testing1234",
    saveSettings: false,
    timezone: "America/Los_Angeles",
    username: "testuser",
    wifiCountry: "US",
    wifiPassword: "testing3456",
    wifiSSID: "test-ssid",
    wifiSSIDHidden: false,
  } as CloudInitForm;
  test("should generate user-data", () => {
    const generator = new CloudInitGenerator(baseFormData);
    const yamlData = generator.generateUserData();

    expect(yamlData).toMatchSnapshot();
  });

  test("should hash password", () => {
    const hashed = CloudInitGenerator.hashPassword(baseFormData.password);
    expect(CloudInitGenerator.comparePassword(baseFormData.password, hashed)).toEqual(true);
  });
});

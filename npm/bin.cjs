const os = require("os");
const path = require("path");
const fs = require("fs");
const { execSync, spawnSync } = require("child_process");
const { version } = require("./package.json");
const { Readable } = require("stream");
const { x } = require("tar");

const binaryName = os.platform() === "win32" ? "lambda.exe" : "lambda";
const binaryPath = path.join(__dirname, binaryName);

async function install() {
  if (fs.existsSync(binaryPath)) return;

  const target = getTarget();
  const url = `https://github.com/aanesn/lambda/releases/download/v${version}/lambda-${target}.tar.gz`;

  let res = await fetch(url);
  if (!res.ok) {
    throw new Error(`failed to fetch release: ${res.statusText}`);
  }

  const stream = Readable.fromWeb(res.body).pipe(x({ C: __dirname }));

  await new Promise((resolve, reject) => {
    stream.on("finish", resolve);
    stream.on("error", reject);
  });

  if (os.platform() !== "win32") {
    fs.chmodSync(binaryPath, 0o755);
  }
}

async function run() {
  if (!fs.existsSync(binaryPath)) await install();

  const args = process.argv.slice(2);

  const child = spawnSync(binaryPath, args, {
    cwd: process.cwd(),
    stdio: "inherit",
  });

  if (child.error) {
    console.error(child.error);
    process.exit(1);
  }

  process.exit(child.status);
}

function getTarget() {
  const type = os.type();
  const arch = getArch();

  switch (type) {
    case "Darwin":
      return `${arch}-apple-darwin`;
    case "Linux":
      const libc = isMusl() ? "musl" : "gnu";
      return `${arch}-unknown-linux-${libc}`;
    case "Windows_NT":
      return `${arch}-pc-windows-msvc`;
    default:
      throw new Error(`unsupported os type: ${type}`);
  }
}

function getArch() {
  const arch = os.arch();
  switch (arch) {
    case "x64":
      return "x86_64";
    case "arm64":
      return "aarch64";
    default:
      throw new Error(`${arch} is an unsupported architecture`);
  }
}

function isMusl() {
  try {
    if (process.report) {
      const rawReport = process.report.getReport();
      const report =
        typeof rawReport === "string" ? JSON.parse(rawReport) : rawReport;

      if (!report || !(report.sharedObjects instanceof Array)) {
        return false;
      }

      return report.sharedObjects.some(
        (o) => o.includes("libc.musl-") || o.includes("ld-musl-"),
      );
    }

    const output = execSync("ldd --version 2>&1 || true", { encoding: "utf8" });
    if (output.includes("musl")) {
      return true;
    }

    return false;
  } catch (error) {
    return false;
  }
}

module.exports = { install, run };

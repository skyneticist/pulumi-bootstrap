import { existsSync, mkdirSync, copyFileSync } from 'fs';
import { dirname, resolve } from 'path';
import { platform } from 'os';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const projectRootDir = resolve(__dirname, '..');
const binaryFileName = platform() === 'win32' ? 'pulumi-bootstrap.exe' : 'pulumi-bootstrap';

const sourceBinaryPath = process.env.BINARY_SOURCE_PATH || resolve(projectRootDir, 'target', 'release', binaryFileName);
if (!existsSync(sourceBinaryPath)) {
    console.error(`Binary source file does not exist: ${sourceBinaryPath}`);
    process.exit(1);
}

const destinationDir = resolve(projectRootDir, 'bin');
if (!existsSync(destinationDir)) {
    mkdirSync(destinationDir, { recursive: true });
}

try {
    copyFileSync(sourceBinaryPath, resolve(destinationDir, binaryFileName));
    console.log(`Rust binary moved to ${destinationDir}`);
} catch (err) {
    console.error(`Error copying binary from ${sourceBinaryPath} to ${destinationDir}: ${err.message}`);
    process.exit(1);
}

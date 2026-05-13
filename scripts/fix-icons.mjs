import sharp from 'sharp';
import { mkdirSync } from 'fs';

mkdirSync('src-tauri/icons', { recursive: true });

const size32 = Buffer.alloc(32 * 32 * 4, 0);
const size128 = Buffer.alloc(128 * 128 * 4, 0);

await sharp(size32, { raw: { width: 32, height: 32, channels: 4 } })
    .png().toFile('src-tauri/icons/tray-icon.png');

await sharp(size32, { raw: { width: 32, height: 32, channels: 4 } })
    .png().toFile('src-tauri/icons/32x32.png');

await sharp(size128, { raw: { width: 128, height: 128, channels: 4 } })
    .png().toFile('src-tauri/icons/128x128.png');

await sharp(size128, { raw: { width: 128, height: 128, channels: 4 } })
    .png().toFile('src-tauri/icons/128x128@2x.png');

console.log('Icons created successfully');
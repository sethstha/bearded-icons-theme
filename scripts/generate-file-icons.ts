import * as fs from 'fs';
import * as path from 'path';

const ICONS_DIR = path.join(__dirname, '..', 'icons');
const OUTPUT_FILE = path.join(__dirname, '..', 'icon_themes', 'file-icons.json');

interface FileIcon {
  path: string;
}

function generateFileIcons(): void {
  try {
    // Read all SVG files from icons directory
    const files = fs.readdirSync(ICONS_DIR)
      .filter(file => file.endsWith('.svg'));

    // Create the file_icons configuration
    const fileIcons: Record<string, FileIcon> = {};

    files.forEach(file => {
      const iconName = path.basename(file, '.svg');
      fileIcons[iconName] = {
        path: `./icons/${file}`
      };
    });

    // Write to a new JSON file
    fs.writeFileSync(
      OUTPUT_FILE,
      JSON.stringify(fileIcons, null, 2),
      'utf8'
    );

    console.log('File icons configuration has been written to file-icons.json');
  } catch (error) {
    console.error('Error generating file icons:', error);
  }
}

generateFileIcons();
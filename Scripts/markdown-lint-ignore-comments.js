import { readFileSync, readdirSync, statSync } from 'fs-extra';
import { join } from 'path';

const MAX_LENGTH = 120;

function isComment(line) {
  return /^\s*<!--[\s\S]*?-->\s*$/.test(line);
}

function checkFile(filePath) {
  const lines = readFileSync(filePath, 'utf8').split('\n');
  lines.forEach((line, idx) => {
    if (!isComment(line) && line.length > MAX_LENGTH) {
      console.log(`${filePath}:${idx + 1}: Line exceeds ${MAX_LENGTH} characters`);
    }
  });
}

// Recursively find all .md files
function findMarkdownFiles(dir) {
  let results = [];
  readdirSync(dir).forEach(file => {
    const fullPath = join(dir, file);
    if (statSync(fullPath).isDirectory()) {
      results = results.concat(findMarkdownFiles(fullPath));
    } else if (file.endsWith('.md')) {
      results.push(fullPath);
    }
  });
  return results;
}

// Usage: node scripts/markdown-lint-ignore-comments.js [directory]
const targetDir = process.argv[2] || '.';
findMarkdownFiles(targetDir).forEach(checkFile);

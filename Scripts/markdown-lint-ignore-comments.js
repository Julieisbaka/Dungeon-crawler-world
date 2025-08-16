import { readFileSync, readdirSync, statSync } from "fs";
import { join, resolve } from "path";

const MAX_LENGTH = 120;

function isComment(line) {
  return /^\s*<!--[\s\S]*?-->\s*$/.test(line);
}

function checkFile(filePath) {
  const lines = readFileSync(filePath, "utf8").split("\n");
  lines.forEach((line, idx) => {
    if (!isComment(line) && line.length > MAX_LENGTH) {
      console.log(
        `${filePath}:${idx + 1}: Line exceeds ${MAX_LENGTH} characters`,
      );
    }
  });
}

// Recursively find all .md files
function findMarkdownFiles(dir) {
  let results = [];
  readdirSync(dir).forEach((file) => {
    const fullPath = join(dir, file);
    if (statSync(fullPath).isDirectory()) {
      results = results.concat(findMarkdownFiles(fullPath));
    } else if (file.endsWith(".md")) {
      results.push(fullPath);
    }
  });
  return results;
}

const ROOT_DIR = process.cwd(); // Define a safe root directory
const targetDir = process.argv[2] ? resolve(process.argv[2]) : ROOT_DIR;

// Ensure the target directory is within the safe root directory
if (!targetDir.startsWith(ROOT_DIR)) {
  console.error(
    `Error: The specified directory is outside the allowed root directory: ${ROOT_DIR}`,
  );
  process.exit(1);
}

findMarkdownFiles(targetDir).forEach(checkFile);

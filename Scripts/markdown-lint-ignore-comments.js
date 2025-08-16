import { readFileSync, readdirSync, statSync } from "fs";
import { join, resolve } from "path";

const MAX_LENGTH = 120;

function isComment(line) {
  return /^\s*<!--[\s\S]*?-->\s*$/.test(line);
}

function checkFile(filePath) {
  // Ensure the file is within the allowed root directory
  let absPath;
  try {
    absPath = require("fs").realpathSync(filePath);
  } catch (e) {
    console.warn(`Warning: Could not resolve path for ${filePath}: ${e.message}`);
    return;
  }
  if (!absPath.startsWith(ROOT_DIR)) {
    console.warn(`Warning: Skipping file outside root: ${filePath}`);
    return;
  }
  const lines = readFileSync(absPath, "utf8").split("\n");
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
    let realFullPath;
    try {
      realFullPath = require("fs").realpathSync(fullPath);
    } catch (e) {
      // Skip files/dirs we can't resolve
      return;
    }
    if (!realFullPath.startsWith(ROOT_DIR)) {
      // Skip files/dirs outside the root
      return;
    }
    if (statSync(realFullPath).isDirectory()) {
      results = results.concat(findMarkdownFiles(realFullPath));
    } else if (file.endsWith(".md")) {
      results.push(realFullPath);
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

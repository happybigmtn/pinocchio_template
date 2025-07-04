import * as codama from "codama";
import * as anchorIdl from "@codama/nodes-from-anchor";
import path from "path";
import * as renderers from "@codama/renderers";
import fs from "fs";
import { fileURLToPath } from "url";
import { dirname } from "path";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Get program name from command line argument
const programName = process.argv[2];
if (!programName) {
  console.error("Usage: node generate-clients.js <program-name>");
  process.exit(1);
}

console.log(`Generating TypeScript client for ${programName}...`);

const projectRoot = path.join(__dirname, "..");
const idlDir = path.join(projectRoot, "idl");
const idlFile = path.join(idlDir, `${programName.replace(/-/g, "_")}.json`);

// Check if IDL file exists
if (!fs.existsSync(idlFile)) {
  console.error(`IDL file not found: ${idlFile}`);
  console.error("Run the IDL generation first:");
  console.error(`npm run gen:idl:${programName}`);
  process.exit(1);
}

const idl = JSON.parse(fs.readFileSync(idlFile, 'utf8'));

// Convert underscores to camelCase for client directory
const clientDirName = programName.replace(/-/g, "").replace(/_([a-z])/g, (match, letter) => letter.toUpperCase());
const jsClientsDir = path.join(projectRoot, "clients", clientDirName);

console.log(`Reading IDL from: ${idlFile}`);
console.log(`Generating client to: ${jsClientsDir}`);

const codamaInstance = codama.createFromRoot(anchorIdl.rootNodeFromAnchor(idl));

// Generate TypeScript client
codamaInstance.accept(
  renderers.renderJavaScriptVisitor(jsClientsDir, {
    formatCode: true,
    deleteFolderBeforeRendering: true,
    prettierOptions: {
      parser: 'typescript',
      singleQuote: true,
      trailingComma: 'all',
      printWidth: 80,
    },
  })
);

console.log(`✅ TypeScript client generated successfully!`);
console.log(`📁 Client location: ${jsClientsDir}`);

// List generated files
if (fs.existsSync(jsClientsDir)) {
  console.log("\n📄 Generated files:");
  const walkDir = (dir, prefix = "") => {
    const files = fs.readdirSync(dir, { withFileTypes: true });
    files.forEach(file => {
      const fullPath = path.join(dir, file.name);
      const relativePath = path.relative(jsClientsDir, fullPath);
      if (file.isDirectory()) {
        console.log(`${prefix}📁 ${relativePath}/`);
        walkDir(fullPath, prefix + "  ");
      } else {
        console.log(`${prefix}📄 ${relativePath}`);
      }
    });
  };
  walkDir(jsClientsDir);
}

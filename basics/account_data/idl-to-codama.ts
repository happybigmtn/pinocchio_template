import { readFileSync, writeFileSync } from 'fs';
import { join } from 'path';

// Read the generated IDL
const idlPath = join(__dirname, 'account_data.json');
const idl = JSON.parse(readFileSync(idlPath, 'utf8'));

// Generate Codama node TypeScript code
const codamaCode = `import {
  rootNode,
  programNode,
  instructionNode,
  constantDiscriminatorNode,
  constantValueNode,
  numberTypeNode,
  numberValueNode,
  instructionArgumentNode,
  instructionAccountNode,
  publicKeyValueNode,
  accountNode,
  structTypeNode,
  structFieldTypeNode,
  fixedSizeTypeNode,
  bytesTypeNode,
} from 'codama';

export const root = rootNode(
  programNode({
    name: '${idl.name}',
    publicKey: '${idl.metadata.address}',
    version: '${idl.version}',
    accounts: [
${idl.accounts.map((account: any) => `      accountNode({
        name: '${account.name.toLowerCase()}',
        data: structTypeNode([
${account.type.fields.map((field: any) => {
  if (field.type.array) {
    return `          structFieldTypeNode({
            name: '${field.name}',
            type: fixedSizeTypeNode(bytesTypeNode(), ${field.type.array[1]}),
          })`;
  } else {
    return `          structFieldTypeNode({
            name: '${field.name}',
            type: numberTypeNode('${field.type}'),
          })`;
  }
}).join(',\n')}
        ]),
      })`).join(',\n')}
    ],
    instructions: [
${idl.instructions.map((instruction: any) => {
  const instructionData = idl.types.find((t: any) => 
    t.name.toLowerCase().includes(instruction.name.toLowerCase())
  );
  
  return `      instructionNode({
        name: '${instruction.name.toLowerCase()}',
        discriminators: [
          constantDiscriminatorNode(
            constantValueNode(numberTypeNode('u8'), numberValueNode(${instruction.discriminant.value}))
          ),
        ],
        arguments: [
          instructionArgumentNode({
            name: 'discriminator',
            type: numberTypeNode('u8'),
            defaultValue: numberValueNode(${instruction.discriminant.value}),
            defaultValueStrategy: 'omitted',
          }),
${instructionData ? instructionData.type.fields.map((field: any) => {
  if (field.type.array) {
    return `          instructionArgumentNode({
            name: '${field.name}',
            type: fixedSizeTypeNode(bytesTypeNode(), ${field.type.array[1]}),
            docs: ['The ${field.name} field'],
          })`;
  } else {
    return `          instructionArgumentNode({
            name: '${field.name}',
            type: numberTypeNode('${field.type}'),
            docs: ['The ${field.name} field'],
          })`;
  }
}).join(',\n') : ''}
        ],
        accounts: [
          instructionAccountNode({
            name: 'owner',
            isSigner: true,
            isWritable: true,
            docs: ['The account that will pay for the transaction'],
          }),
          instructionAccountNode({
            name: 'addressInfo',
            isSigner: true,
            isWritable: true,
            docs: ['The address info account to create'],
          }),
          instructionAccountNode({
            name: 'systemProgram',
            defaultValue: publicKeyValueNode(
              '11111111111111111111111111111111',
              'systemProgram'
            ),
            isSigner: false,
            isWritable: false,
            docs: ['System Program'],
          }),
        ],
      })`;
}).join(',\n')}
    ],
  })
);
`;

// Write the generated Codama node
const outputPath = join(__dirname, 'codama-node-generated.ts');
writeFileSync(outputPath, codamaCode);

console.log(`Generated Codama node at ${outputPath}`);

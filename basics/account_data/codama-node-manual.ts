import {
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
    name: 'accountData',
    publicKey: 'EAUvJAw61MTaJbyV4tqFB4dEZuYHdYrtpGQ35hDsQ6Dw',
    version: '1.0.0',
    accounts: [
      accountNode({
        name: 'addressInfo',
        data: structTypeNode([
          structFieldTypeNode({
            name: 'name',
            type: fixedSizeTypeNode(bytesTypeNode(), 50),
          }),
          structFieldTypeNode({
            name: 'houseNumber',
            type: numberTypeNode('u8'),
          }),
          structFieldTypeNode({
            name: 'street',
            type: fixedSizeTypeNode(bytesTypeNode(), 50),
          }),
          structFieldTypeNode({
            name: 'city',
            type: fixedSizeTypeNode(bytesTypeNode(), 50),
          }),
        ]),
      }),
    ],
    instructions: [
      instructionNode({
        name: 'create',
        discriminators: [
          constantDiscriminatorNode(
            constantValueNode(numberTypeNode('u8'), numberValueNode(0))
          ),
        ],
        arguments: [
          instructionArgumentNode({
            name: 'discriminator',
            type: numberTypeNode('u8'),
            defaultValue: numberValueNode(0),
            defaultValueStrategy: 'omitted',
          }),
          instructionArgumentNode({
            name: 'name',
            type: fixedSizeTypeNode(bytesTypeNode(), 50),
            docs: ['The name field'],
          }),
          instructionArgumentNode({
            name: 'houseNumber',
            type: numberTypeNode('u8'),
            docs: ['The house number'],
          }),
          instructionArgumentNode({
            name: 'street',
            type: fixedSizeTypeNode(bytesTypeNode(), 50),
            docs: ['The street field'],
          }),
          instructionArgumentNode({
            name: 'city',
            type: fixedSizeTypeNode(bytesTypeNode(), 50),
            docs: ['The city field'],
          }),
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
      }),
    ],
  })
);


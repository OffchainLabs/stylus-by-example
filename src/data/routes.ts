export type BasicExamples = (typeof basicExamples)[number];
export type RouteInfo = {
  route: string;
  title: string;
  description: string;
};

export const gettingStarted = [
  {
    route: '/getting_started/using_the_cli',
    title: 'Using the CLI',
    description: "How to use cargo stylus and Foundry's cast to effectively test smart contracts",
  },
];

export const basicExamples = [
  {
    route: '/basic_examples/hello_world',
    title: 'Hello World',
    description: 'Learn how to use the console output',
  },
  {
    route: '/basic_examples/bytes_in_bytes_out',
    title: 'Bytes In, Bytes Out',
    description: 'Learn about the basic entrypoint model',
  },
  {
    route: '/basic_examples/first_app',
    title: 'First App',
    description:
      'Simple counter contract that defines a count value in storage that can be incremented',
  },
  {
    route: '/basic_examples/primitive_data_types',
    title: 'Primitive Data Types',
    description: 'Learn about booleans, integers, and addresses',
  },
  {
    route: '/basic_examples/storage_data_types',
    title: 'Storage Data Types',
    description: 'Defines the basic state variables used in Arbitrum Stylus Rust smart contracts and how they can be used to store and retrieve state from the VM.',
  },
  {
    route: '/basic_examples/variables',
    title: 'Variables',
    description: 'Learn about local, state, and global variables',
  },
  {
    route: '/basic_examples/constants',
    title: 'Constants',
    description: 'How to define constant values in your contract',
  },
  {
    route: '/basic_examples/constructor',
    title: 'Constructor',
    description: 'Learn about the constructor and how to initialize your contract',
  },
  {
    route: '/basic_examples/mapping',
    title: 'Mapping',
    description: 'Learn about the mapping data structure',
  },
  {
    route: '/basic_examples/arrays',
    title: 'Arrays',
    description: 'A simple arrays example in stylus',
  },
  {
    route: '/basic_examples/events',
    title: 'Events',
    description: 'Log public events to the blockchain',
  },
  {
    route: '/basic_examples/call',
    title: 'Call',
    description: 'Call other contracts on Rust Stylus smart contracts',
  },
  {
    route: '/basic_examples/errors',
    title: 'Errors',
    description: 'Errors on Stylus Rust smart contracts',
  },
  {
    route: '/basic_examples/test',
    title: 'Test',
    description: 'Test on Stylus Rust smart contracts',
  },
  {
    route: "/basic_examples/sending_ether",
    title: "Sending Ether",
    description: "Sending Ethers on Stylus Rust smart contracts",
  },
  {
    route: "/basic_examples/function_selector",
    title: "Function selector",
    description: "Compute the encoded function selector of a contract's function",
  },
    {
    route: "/basic_examples/fallback_receive",
    title: "Fallback and Receive",
    description: "Learn about the fallback and receive functions in Stylus Rust smart contracts",
  },
  {
    route: "/basic_examples/verify_signature",
    title: "Verifying Signature",
    description:
      "An explanation of how to verify a signature signed off chain.",
  },
  {
    route: "/basic_examples/inheritance",
    title: "Inheritance",
    description: "Inheritance in the Stylus Rust SDK",
  },
  {
    route: "/basic_examples/import_interfaces",
    title: "Import interfaces",
    description: "Import external contract interfaces",
  },
  {
    route: "/basic_examples/export_interface",
    title: "Export contract interface",
    description: "Export the Solidity ABI interface of your Rust contract",
  },
  {
    route: "/basic_examples/vm_affordances",
    title: "VM affordances",
    description: "Accesing VM affordances in the Stylus Rust SDK",
  },
  {
    route: "/basic_examples/abi_encode",
    title: "ABI Encode",
    description:
      "A simple solidity ABI encode and decode example",
  },
  {
    route: "/basic_examples/abi_decode",
    title: "ABI Decode",
    description:
      "A simple solidity ABI encode and decode example",
  },
  {
    route: "/basic_examples/hashing",
    title: "Hashing with keccak256",
    description:
      "A simple solidity ABI encode and decode example",
  },
  {
    route: '/basic_examples/function',
    title: 'Functions',
    description: 'Learn about function parameters, return types, and visibility',
  },
];

export const applications = [
  {
    route: '/applications/erc20',
    title: 'ERC-20',
    description: 'An example implementation of the ERC-20 token standard in Rust',
  },
  {
    route: '/applications/erc721',
    title: 'ERC-721',
    description: 'An example implementation of the ERC-721 token standard in Rust',
  },
  {
    route: "/applications/time_lock",
    title: "Time Lock",
    description: "An example implementation of the Timelock wallet in Rust using Arbitrum Stylus.",
  },
  {
    route: "/applications/multi_call",
    title: "Multi Call",
    description: "An example implementation of the Multi Call contract in Rust using Arbitrum Stylus",
  },
  {
    route: "/applications/english_auction",
    title: "English Auction",
    description: "An example implementation of the English Auction in Rust using Arbitrum Stylus.",
  },
  {
    route: '/applications/multi_sig',
    title: 'MultiSig',
    description: 'An example implementation of the MultiSig wallet in Rust using Arbitrum Stylus.',
  },
  {
    route: '/applications/vending_machine',
    title: 'Vending Machine',
    description: 'An example implementation of the Vending Machine in Rust using Arbitrum Stylus.',
  },
];

export type BasicExamples = (typeof basicExamples)[number];
export type RouteInfo = {
  route: string;
  title: string;
  description: string;
};

export const gettingStarted = [
  {
    route: "/getting_started/using_the_cli",
    title: "Using the CLI",
    description:
      "How to use cargo stylus and Foundry's cast to effectively test smart contracts",
  },
];

export const basicExamples = [
  {
    route: "/basic_examples/hello_world",
    title: "Hello World",
    description: "Learn how to use the console output",
  },
  {
    route: "/basic_examples/bytes_in_bytes_out",
    title: "Bytes In, Bytes Out",
    description: "Learn about the basic entrypoint model",
  },
  {
    route: "/basic_examples/first_app",
    title: "First App",
    description:
      "Simple counter contract that defines a count value in storage that can be incremented",
  },
  {
    route: "/basic_examples/primitive_data_types",
    title: "Primitive Data Types",
    description: "Learn about booleans, integers, and addresses",
  },
  {
    route: "/basic_examples/variables",
    title: "Variables",
    description: "Learn about local, state, and global variables",
  },
  {
    route: "/basic_examples/constants",
    title: "Constants",
    description: "How to define constant values in your contract",
  },
  {
    route: "/basic_examples/events",
    title: "Events",
    description: "Log public events to the blockchain",
  },
  {
    route: "/basic_examples/errors",
    title: "Errors",
    description: "Errors on Stylus Rust smart contracts",
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
    route: "/basic_examples/inheritance",
    title: "Inheritance",
    description: "Inheritance in the Stylus Rust SDK",
  },
  {
    route: "/basic_examples/vm_affordances",
    title: "VM affordances",
    description: "Accesing VM affordances in the Stylus Rust SDK",
  },
];

export const applications = [
  {
    route: "/applications/erc20",
    title: "ERC-20",
    description: "An example implementation of the ERC-20 token standard in Rust",
  },
  {
    route: "/applications/erc721",
    title: "ERC-721",
    description: "An example implementation of the ERC-721 token standard in Rust",
  },
  {
    route: "/applications/english_auction",
    title: "English Auction",
    description: "An example implementation of the English Auction in Rust using Arbitrum Stylus.",
  },
  {
    route: "/applications/multi_sig",
    title: "MultiSig",
    description: "An example implementation of the MultiSig wallet in Rust using Arbitrum Stylus.",
  },
  {
    route: "/applications/vending_machine",
    title: "Vending Machine",
    description: "An example implementation of the Vending Machine in Rust using Arbitrum Stylus.",
  },
];

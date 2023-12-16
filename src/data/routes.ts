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
];

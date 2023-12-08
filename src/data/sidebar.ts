export type BasicExamples = (typeof basicExamples)[number];
export type RouteInfo = {
  route: string;
  title: string;
  description: string;
};

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
    route: "/",
    title: "First App",
    description: "Learn how to use the console output",
  },
  {
    route: "/",
    title: "Primitive Data Types",
    description: "Learn how to use the console output",
  },
  {
    route: "/",
    title: "Variables",
    description: "Learn how to use the console output",
  },
  {
    route: "/",
    title: "Immutable",
    description: "Learn how to use the console output",
  },
];

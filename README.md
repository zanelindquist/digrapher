# Digrapher
Digrapher is a discrete math toolchain that helps users visualize concepts like relations, matricies, and other data strucutes.

The term *digraph* refers to [directed graphs](https://en.wikipedia.org/wiki/Directed_graph):
>In mathematics, and more specifically in graph theory, a directed graph (or digraph) is a graph that is made up of a set of vertices connected by directed edges, often called arcs.

<img src="assets/demo_images/digraph.png" alt="digraph" style="height: 150px; justify-self: center;"/>
<img src="assets/demo_images/matrix.png" alt="matrix" style="height: 150px; justify-self: center;"/>

## Key Features
- Visualize relations on sets with digraph diagrams
- Inspect relations in matrix view
- View tree structures
- Build relations

<img src="assets/demo_images/demo.png" alt="project demo" style="height: 300px; justify-self: center;"/>

## Deploy Locally
Digrapher can be deployed locally through a Docker Virtual Machine
1. Install [Docker](https://www.docker.com/)
2. Run `docker run -p 8080:80 zanelindquist/digrapher`
3. Navigate to [localhost:8080](localhost:8080)

## Contributing
Contributions are gladly appreciated! Please view [current issues](https://github.com/zanelindquist/digrapher/issues), or feel free to [create a new issue](https://github.com/zanelindquist/digrapher/issues/new/choose)!

For more information, refer to [CONTRIBUTING.md](CONTRIBUTING.md)

## Tech Stack
- **Rust** utilizing **Yew**, which builds the app into **WASM**
- **Docker Container** to serve the application locally
module.exports = {
  devServer: {
    proxy: [
      {
        context: ["/graphql", "/login", "/logout", "/auth_callback"],
        target: "http://127.0.0.1:8080",
      },
    ],
    devMiddleware: {
      writeToDisk: true,
    },
  },
};

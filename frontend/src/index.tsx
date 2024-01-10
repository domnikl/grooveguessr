import ReactDOM from "react-dom/client";
import "@fontsource/moirai-one";
import "./index.css";
import App from "./App";
import reportWebVitals from "./reportWebVitals";
import {
  ApolloClient,
  InMemoryCache,
  ApolloProvider,
  HttpLink,
} from "@apollo/client";
import { onError } from "@apollo/client/link/error";

const loginRedirectLink = onError(({ networkError }: any) => {
  if (networkError?.statusCode === 401) {
    window.location.href = "/login";
  }
});

const httpLink = new HttpLink({ uri: "/graphql" });

const root = ReactDOM.createRoot(
  document.getElementById("root") as HTMLElement
);
const client = new ApolloClient({
  cache: new InMemoryCache(),
  link: loginRedirectLink.concat(httpLink),
});

root.render(
  <ApolloProvider client={client}>
    <App />
  </ApolloProvider>
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();

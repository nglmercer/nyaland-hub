import { h, render } from "preact";
import { App } from "./App";
import "./styles/app.css";

render(h(App, null), document.getElementById("app")!);

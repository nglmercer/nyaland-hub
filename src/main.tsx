import { h, render } from "preact";
import { App } from "./App";
import "./styles/base.css";
import "./styles/layout.css";
import "./styles/search.css";
import "./styles/table.css";
import "./styles/detail.css";
import "./styles/downloads.css";
import "./styles/settings.css";
import "./styles/responsive.css";

render(h(App, null), document.getElementById("app")!);
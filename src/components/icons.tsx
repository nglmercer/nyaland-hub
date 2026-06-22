import { h } from "preact";

interface IconProps {
  size?: number;
  class?: string;
}

export function SearchIcon({ size = 18, class: cls }: IconProps) {
  return h("svg", {
    width: size,
    height: size,
    viewBox: "0 0 24 24",
    fill: "none",
    stroke: "currentColor",
    "stroke-width": 2,
    "stroke-linecap": "round",
    "stroke-linejoin": "round",
    class: cls,
  },
    h("circle", { cx: 11, cy: 11, r: 8 }),
    h("line", { x1: 21, y1: 21, x2: 16.65, y2: 16.65 }),
  );
}

export function DownloadIcon({ size = 18, class: cls }: IconProps) {
  return h("svg", {
    width: size,
    height: size,
    viewBox: "0 0 24 24",
    fill: "none",
    stroke: "currentColor",
    "stroke-width": 2,
    "stroke-linecap": "round",
    "stroke-linejoin": "round",
    class: cls,
  },
    h("path", { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }),
    h("polyline", { points: "7 10 12 15 17 10" }),
    h("line", { x1: 12, y1: 15, x2: 12, y2: 3 }),
  );
}

export function SettingsIcon({ size = 18, class: cls }: IconProps) {
  return h("svg", {
    width: size,
    height: size,
    viewBox: "0 0 24 24",
    fill: "none",
    stroke: "currentColor",
    "stroke-width": 2,
    "stroke-linecap": "round",
    "stroke-linejoin": "round",
    class: cls,
  },
    h("circle", { cx: 12, cy: 12, r: 3 }),
    h("path", { d: "M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z" }),
  );
}

export function PauseIcon({ size = 16, class: cls }: IconProps) {
  return h("svg", {
    width: size,
    height: size,
    viewBox: "0 0 24 24",
    fill: "currentColor",
    class: cls,
  },
    h("rect", { x: 6, y: 4, width: 4, height: 16 }),
    h("rect", { x: 14, y: 4, width: 4, height: 16 }),
  );
}

export function PlayIcon({ size = 16, class: cls }: IconProps) {
  return h("svg", {
    width: size,
    height: size,
    viewBox: "0 0 24 24",
    fill: "currentColor",
    class: cls,
  },
    h("polygon", { points: "5,3 19,12 5,21" }),
  );
}

export function CloseIcon({ size = 16, class: cls }: IconProps) {
  return h("svg", {
    width: size,
    height: size,
    viewBox: "0 0 24 24",
    fill: "none",
    stroke: "currentColor",
    "stroke-width": 2,
    "stroke-linecap": "round",
    "stroke-linejoin": "round",
    class: cls,
  },
    h("line", { x1: 18, y1: 6, x2: 6, y2: 18 }),
    h("line", { x1: 6, y1: 6, x2: 18, y2: 18 }),
  );
}

export function ChevronLeftIcon({ size = 16, class: cls }: IconProps) {
  return h("svg", {
    width: size,
    height: size,
    viewBox: "0 0 24 24",
    fill: "none",
    stroke: "currentColor",
    "stroke-width": 2,
    "stroke-linecap": "round",
    "stroke-linejoin": "round",
    class: cls,
  },
    h("polyline", { points: "15 18 9 12 15 6" }),
  );
}

export function ChevronRightIcon({ size = 16, class: cls }: IconProps) {
  return h("svg", {
    width: size,
    height: size,
    viewBox: "0 0 24 24",
    fill: "none",
    stroke: "currentColor",
    "stroke-width": 2,
    "stroke-linecap": "round",
    "stroke-linejoin": "round",
    class: cls,
  },
    h("polyline", { points: "9 18 15 12 9 6" }),
  );
}

export function TrashIcon({ size = 16, class: cls }: IconProps) {
  return h("svg", {
    width: size,
    height: size,
    viewBox: "0 0 24 24",
    fill: "none",
    stroke: "currentColor",
    "stroke-width": 2,
    "stroke-linecap": "round",
    "stroke-linejoin": "round",
    class: cls,
  },
    h("polyline", { points: "3 6 5 6 21 6" }),
    h("path", { d: "M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" }),
  );
}

export function MagnetIcon({ size = 16, class: cls }: IconProps) {
  return h("svg", {
    width: size,
    height: size,
    viewBox: "0 0 24 24",
    fill: "none",
    stroke: "currentColor",
    "stroke-width": 2,
    "stroke-linecap": "round",
    "stroke-linejoin": "round",
    class: cls,
  },
    h("path", { d: "M6 3v7a6 6 0 0 0 6 6 6 6 0 0 0 6-6V3" }),
    h("line", { x1: 4, y1: 21, x2: 4, y2: 14 }),
    h("line", { x1: 10, y1: 21, x2: 10, y2: 14 }),
    h("line", { x1: 16, y1: 21, x2: 16, y2: 14 }),
    h("line", { x1: 22, y1: 21, x2: 22, y2: 14 }),
  );
}

export function GlobeIcon({ size = 16, class: cls }: IconProps) {
  return h("svg", {
    width: size,
    height: size,
    viewBox: "0 0 24 24",
    fill: "none",
    stroke: "currentColor",
    "stroke-width": 2,
    "stroke-linecap": "round",
    "stroke-linejoin": "round",
    class: cls,
  },
    h("circle", { cx: 12, cy: 12, r: 10 }),
    h("line", { x1: 2, y1: 12, x2: 22, y2: 12 }),
    h("path", { d: "M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z" }),
  );
}

export function SunIcon({ size = 16, class: cls }: IconProps) {
  return h("svg", {
    width: size,
    height: size,
    viewBox: "0 0 24 24",
    fill: "none",
    stroke: "currentColor",
    "stroke-width": 2,
    "stroke-linecap": "round",
    "stroke-linejoin": "round",
    class: cls,
  },
    h("circle", { cx: 12, cy: 12, r: 5 }),
    h("line", { x1: 12, y1: 1, x2: 12, y2: 3 }),
    h("line", { x1: 12, y1: 21, x2: 12, y2: 23 }),
    h("line", { x1: 4.22, y1: 4.22, x2: 5.64, y2: 5.64 }),
    h("line", { x1: 18.36, y1: 18.36, x2: 19.78, y2: 19.78 }),
    h("line", { x1: 1, y1: 12, x2: 3, y2: 12 }),
    h("line", { x1: 21, y1: 12, x2: 23, y2: 12 }),
    h("line", { x1: 4.22, y1: 19.78, x2: 5.64, y2: 18.36 }),
    h("line", { x1: 18.36, y1: 5.64, x2: 19.78, y2: 4.22 }),
  );
}

export function MoonIcon({ size = 16, class: cls }: IconProps) {
  return h("svg", {
    width: size,
    height: size,
    viewBox: "0 0 24 24",
    fill: "none",
    stroke: "currentColor",
    "stroke-width": 2,
    "stroke-linecap": "round",
    "stroke-linejoin": "round",
    class: cls,
  },
    h("path", { d: "M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" }),
  );
}

export function FilterIcon({ size = 16, class: cls }: IconProps) {
  return h("svg", {
    width: size,
    height: size,
    viewBox: "0 0 24 24",
    fill: "none",
    stroke: "currentColor",
    "stroke-width": 2,
    "stroke-linecap": "round",
    "stroke-linejoin": "round",
    class: cls,
  },
    h("polygon", { points: "22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3" }),
  );
}

export function ChevronDownIcon({ size = 16, class: cls }: IconProps) {
  return h("svg", {
    width: size,
    height: size,
    viewBox: "0 0 24 24",
    fill: "none",
    stroke: "currentColor",
    "stroke-width": 2,
    "stroke-linecap": "round",
    "stroke-linejoin": "round",
    class: cls,
  },
    h("polyline", { points: "6 9 12 15 18 9" }),
  );
}

export function FolderOpenIcon({ size = 16, class: cls }: IconProps) {
  return h("svg", {
    width: size,
    height: size,
    viewBox: "0 0 24 24",
    fill: "none",
    stroke: "currentColor",
    "stroke-width": 2,
    "stroke-linecap": "round",
    "stroke-linejoin": "round",
    class: cls,
  },
    h("path", { d: "M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z" }),
    h("line", { x1: 2, y1: 10, x2: 22, y2: 10 }),
  );
}

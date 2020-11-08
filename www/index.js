import * as wasm from "gpx-web-utils";

const inputElement = document.getElementById("input");

inputElement.addEventListener("change", readFiles, false);

function readFiles() {
  if (inputElement.files.length < 2) { alert("open two or more files"); return; }

  const files = Array.from(inputElement.files);
  const promises = files.map(f => f.text());

  Promise.all(promises).then(gpxes => {
    const merged = wasm.merge(gpxes);

    writeOutput(merged);

    inputElement.value = "";
  });
}

function writeOutput(files) {
    const blob = new Blob([files], {type: "text/gpx"});
    const a = document.createElement("a");
    a.href = URL.createObjectURL(blob);
    a.download = "merged.gpx";
    a.click();
    URL.revokeObjectURL(a.href);
}

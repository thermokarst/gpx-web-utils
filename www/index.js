import * as gpx from "gpx-web-utils";

const inputElement = document.getElementById("gpxInput");

inputElement.addEventListener("change", readFiles, false);

function readFiles() {
  if (inputElement.files.length < 2) { alert("open two or more files"); return; }

  const files = Array.from(inputElement.files);
  const promises = files.map(f => f.text());

  Promise.all(promises).then(gpxes => {
    const merged = gpx.merge(gpxes);
    writeOutput(merged);
    inputElement.value = "";
  });
}

function writeOutput(file) {
    const blob = new Blob([file], {type: "text/gpx"});
    const a = document.createElement("a");
    a.href = URL.createObjectURL(blob);
    a.download = "merged.gpx";
    a.click();
    URL.revokeObjectURL(a.href);
}

import * as gpx from "gpx-web-utils";

const inputElement = document.getElementById("gpxInput");
const loadingElement = document.createElement("span");

loadingElement.innerHTML = "<strong>processing...</strong>";
inputElement.value = "";
inputElement.addEventListener("change", readFiles, false);

function readFiles() {
  if (inputElement.files.length < 2) { alert("open two or more files"); return; }

  inputElement.replaceWith(loadingElement);
  const files = Array.from(inputElement.files);
  const promises = files.map(f => f.text());

  Promise.all(promises).then(gpxes => {
    try {
      const merged = gpx.merge(gpxes);
      writeOutput(merged);
    } catch {
      alert("there was a problem, please check the console.");
    } finally {
      inputElement.value = "";
      loadingElement.replaceWith(inputElement);
    }
  });
}

function writeOutput(file) {
    const blob = new Blob([file], {type: "text/gpx"});
    const anchorElement = document.createElement("a");
    anchorElement.href = URL.createObjectURL(blob);
    anchorElement.download = "merged.gpx";
    anchorElement.click();
    URL.revokeObjectURL(anchorElement.href);
}

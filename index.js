async function getProjects() {
  const response = await fetch("./projects.json");
  return response.json();
}

const entries = document.getElementById("entries");
function renderProject(project) {
  const li = document.createElement("li");
  li.innerHTML = `
    <a href="./built/${project.name}/dist/index.html">
      <h3>${project.name}</h3>
      <p><code>${project.size.total}</code> bytes</p>
      <iframe src="./built/${project.name}/dist/index.html"></iframe>
      <div class="iframe-overlay"></div>
    </a>
  `;

  const iframe = li.querySelector("iframe");
  iframe.addEventListener("load", () => iframe.contentWindow.postMessage({}, "*"));

  entries.appendChild(li);
}

async function main() {
  const projects = await getProjects();
  for (const p of projects) {
    renderProject(p);
  }
}

main();

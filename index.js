async function getProjects() {
  const response = await fetch("./projects.json");
  return response.json();
}

const entries = document.getElementById("entries");
function renderProject(project) {
  const projectUrl = `./built/${project.name}/dist/index.html`;
  const li = document.createElement("li");

  const a = document.createElement("a");
  a.href = projectUrl;

  const h3 = document.createElement("h3");
  h3.textContent = project.name;

  const iframe = document.createElement("iframe");
  iframe.src = projectUrl;
  iframe.addEventListener("load", () => {
    iframe.contentWindow.postMessage({}, "*");
  });

  a.appendChild(h3);
  a.append(iframe);
  li.appendChild(a);
  entries.appendChild(li);
}

async function main() {
  const projects = await getProjects();
  for (const p of projects) {
    renderProject(p);
  }
}

main();

async function getProjects() {
  const response = await fetch("./projects.json");
  return response.json();
}

const entries = document.getElementById("entries");
function renderProject(project) {
  const projectUrl = `./built/${project.name}/dist/index.html`;
  const li = document.createElement("li");
  const h3 = document.createElement("h3");
  const a = document.createElement("a");
  a.textContent = project.name;
  a.href = projectUrl;
  const iframe = document.createElement("iframe");
  iframe.src = projectUrl;
  h3.appendChild(a);
  li.appendChild(h3);
  li.append(iframe);
  entries.appendChild(li);
}

async function main() {
  const projects = await getProjects();
  for (const p of projects) {
    renderProject(p);
  }
}

main();

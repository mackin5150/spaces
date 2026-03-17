const app = document.querySelector<HTMLDivElement>("#app");

if (app) {
  app.innerHTML = `
    <main>
      <h1>{{name}}</h1>
      <p>Vite starter scaffolded by spaces.</p>
    </main>
  `;
}

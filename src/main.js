const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  await invoke("greet", { name: greetInputEl.value });
  greetInputEl.value = "";
  
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.getElementById("launcher");
  greetInputEl.addEventListener("keyup", (e) => {
    if (e.key === "Enter") {
      greet();
    }
  });
});

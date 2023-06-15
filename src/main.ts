import { invoke } from "@tauri-apps/api/tauri";

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;

let primeFacNum: HTMLInputElement | null;
let primeFacList: HTMLElement | null;

async function greet() {
  if (greetMsgEl && greetInputEl) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsgEl.textContent = await invoke("greet", {
      name: greetInputEl.value,
    });
  }
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});

async function primeFactors() {
  console.log("testNum = "+primeFacNum?.value);
  if(primeFacList && primeFacNum){
    //if(typeof primeFacNum.value == "number") { return }
    primeFacList.textContent = await invoke("command_prime_factorization", {
      num: primeFacNum.value,
    });
  }
}

window.addEventListener("DOMContentLoaded", () => {
  primeFacNum = document.querySelector("#primeFac-Input");
  primeFacList = document.querySelector("#primeFac-Show");
  document.querySelector("#primeFac")?.addEventListener("submit", (e) => {
    e.preventDefault();
    primeFactors();
  });
});
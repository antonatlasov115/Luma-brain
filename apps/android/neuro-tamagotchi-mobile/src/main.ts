import { invoke } from "@tauri-apps/api/core";

interface PetState {
  name: string;
  hunger: number;
  energy: number;
  happiness: number;
  stimulation: number;
  age: number;
  is_sleeping: boolean;
  mood: string;
  consciousness_index: number;
  phi: number;
  awareness: number;
  vocabulary_size: number;
  total_interactions: number;
  events: string[];
  pet_speech: string[];
  brainwaves: {
    delta: number;
    theta: number;
    alpha: number;
    beta: number;
    gamma: number;
  };
  chemistry: Array<{ name: string; level: number }>;
  thoughts: string[];
  brain_regions: Array<{ name: string; emoji: string; activation: number }>;
}

let updateInterval: number;

// Получить состояние питомца
async function getPetState(): Promise<PetState> {
  return await invoke("get_pet_state");
}

// Действия
async function feedPet() {
  await invoke("feed_pet");
  await updateUI();
}

async function playWithPet() {
  await invoke("play_with_pet");
  await updateUI();
}

async function studyWithPet() {
  await invoke("study_with_pet");
  await updateUI();
}

async function toggleSleep() {
  await invoke("toggle_sleep");
  await updateUI();
}

// Обновить UI
async function updateUI() {
  try {
    const state = await getPetState();

    // Заголовок
    document.getElementById("pet-name")!.textContent = state.name;
    document.getElementById("age")!.textContent = `${state.age}с`;
    document.getElementById("vocab")!.textContent = `${state.vocabulary_size} слов`;
    document.getElementById("interactions")!.textContent = `${state.total_interactions} взаимодействий`;

    // ASCII арт питомца
    const asciiArt = getAsciiArt(state.is_sleeping, state.happiness);
    document.querySelector(".ascii-art pre")!.textContent = asciiArt;

    // Настроение
    document.getElementById("mood")!.textContent = state.mood;

    // Речь питомца
    const speechContent = state.pet_speech.length > 0
      ? state.pet_speech[state.pet_speech.length - 1]
      : "...";
    document.getElementById("pet-speech")!.textContent = speechContent;

    // Параметры
    updateStat("hunger", state.hunger);
    updateStat("energy", state.energy);
    updateStat("happiness", state.happiness);
    updateStat("stimulation", state.stimulation);

    // Мозговые волны
    updateWave("delta", state.brainwaves.delta);
    updateWave("theta", state.brainwaves.theta);
    updateWave("alpha", state.brainwaves.alpha);
    updateWave("beta", state.brainwaves.beta);
    updateWave("gamma", state.brainwaves.gamma);
    document.getElementById("consciousness-index")!.textContent = state.consciousness_index.toFixed(2);

    // Химия
    const chemistryList = document.getElementById("chemistry-list")!;
    chemistryList.innerHTML = state.chemistry
      .map(
        (chem) => `
        <div class="chemistry-item">
          <span>${chem.name}:</span>
          <div class="wave-bar" style="flex: 1;">
            <div class="wave-fill" style="width: ${chem.level * 100}%"></div>
          </div>
          <span>${Math.round(chem.level * 100)}%</span>
        </div>
      `
      )
      .join("");

    // Сознание
    document.getElementById("awareness")!.textContent = `${Math.round(state.awareness * 100)}%`;
    document.getElementById("phi")!.textContent = state.phi.toFixed(3);

    // Области мозга
    const brainRegions = document.getElementById("brain-regions")!;
    brainRegions.innerHTML = state.brain_regions
      .map(
        (region) => `
        <div class="region-item">
          <span>${region.emoji} ${region.name}</span>
          <span>${Math.round(region.activation * 100)}%</span>
        </div>
      `
      )
      .join("");

    // Мысли
    const thoughtsList = document.getElementById("thoughts-list")!;
    thoughtsList.innerHTML = state.thoughts.length > 0
      ? state.thoughts
          .map((thought) => `<div class="thought-item">${thought}</div>`)
          .join("")
      : '<div class="thought-item">Пока нет мыслей...</div>';

    // События
    const eventsList = document.getElementById("events-list")!;
    eventsList.innerHTML = state.events.length > 0
      ? state.events
          .slice()
          .reverse()
          .map((event) => `<div class="event-item">${event}</div>`)
          .join("")
      : '<div class="event-item">Нет событий</div>';

    // Обновить кнопку сна
    const sleepBtn = document.getElementById("sleep-btn")!;
    sleepBtn.querySelector("span:last-child")!.textContent = state.is_sleeping ? "Разбудить" : "Спать";
  } catch (error) {
    console.error("Ошибка обновления UI:", error);
  }
}

function updateStat(name: string, value: number) {
  const bar = document.getElementById(`${name}-bar`)!;
  const valueEl = document.getElementById(`${name}-value`)!;
  bar.style.width = `${value}%`;
  valueEl.textContent = `${Math.round(value)}%`;
}

function updateWave(name: string, value: number) {
  const bar = document.getElementById(`${name}-bar`)!;
  const valueEl = document.getElementById(`${name}-value`)!;
  bar.style.width = `${value * 100}%`;
  valueEl.textContent = `${Math.round(value * 100)}%`;
}

function getAsciiArt(isSleeping: boolean, happiness: number): string {
  if (isSleeping) {
    return `     zzZ
    zzZ
   (-.-)
   />  <\\`;
  } else if (happiness > 70) {
    return `   (^_^)
   />  <\\
    | |   `;
  } else if (happiness > 40) {
    return `   (o_o)
   />  <\\
    | |   `;
  } else {
    return `   (T_T)
   />  <\\
    | |   `;
  }
}

// Переключение вкладок
function setupTabs() {
  const tabs = document.querySelectorAll(".tab");
  const panels = document.querySelectorAll(".tab-panel");

  tabs.forEach((tab) => {
    tab.addEventListener("click", () => {
      const tabName = tab.getAttribute("data-tab");

      tabs.forEach((t) => t.classList.remove("active"));
      panels.forEach((p) => p.classList.remove("active"));

      tab.classList.add("active");
      document.getElementById(`${tabName}-tab`)?.classList.add("active");
    });
  });
}

// Инициализация
window.addEventListener("DOMContentLoaded", async () => {
  // Настроить вкладки
  setupTabs();

  // Кнопки действий
  document.getElementById("feed-btn")?.addEventListener("click", feedPet);
  document.getElementById("play-btn")?.addEventListener("click", playWithPet);
  document.getElementById("study-btn")?.addEventListener("click", studyWithPet);
  document.getElementById("sleep-btn")?.addEventListener("click", toggleSleep);

  // Первое обновление
  await updateUI();

  // Автообновление каждые 500мс
  updateInterval = window.setInterval(updateUI, 500);
});

// Очистка при выходе
window.addEventListener("beforeunload", () => {
  if (updateInterval) {
    clearInterval(updateInterval);
  }
});

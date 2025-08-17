<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import type { Game } from "../lib";

  let {
    onChange,
  }: {
    onChange?: () => void;
  } = $props();

  let menuOpen = $state(false);
  let newName = $state("");
  let newLaunch = $state("");
  let newDesc = $state("");

  let editing = $state(false);
  let oldGame: Game;

  function startAddGame() {
    menuOpen = true;
    editing = false;
    resetInputs();
  }

  async function addGame() {
    const newGame: Game = {
      name: newName,
      launch_command: newLaunch,
      description: newDesc,
    };
    await invoke("add_game", { game: newGame });
    menuOpen = false;
    onChange?.();
  }

  export function startEditGame(game: Game) {
    menuOpen = true;
    editing = true;
    oldGame = game;

    newName = game.name;
    newLaunch = game.launch_command;
    newDesc = game.description;
  }

  async function editGame() {
    // console.log(game);
    const newGame: Game = {
      name: newName,
      launch_command: newLaunch,
      description: newDesc,
    };
    await invoke("edit_game", { id: oldGame.name, game: newGame });
    menuOpen = false;
    onChange?.();
  }

  function resetInputs() {
    newName = "";
    newLaunch = "";
    newDesc = "";
  }

  function onPresetSelected(event: Event) {
    const target = event.target as HTMLSelectElement;
    console.log(target.value);
    switch (target.value) {
      case "exe":
        // invoke("select_exe")
        break;
      case "flatpak":
        newLaunch =
          "flatpak run com.valvesoftware.Steam steam://run/{STEAM ID}";
        break;
      case "steam":
        newLaunch = "steam steam://run/{STEAM ID}";
        break;
    }
  }
</script>

<button
  class="w-32 cursor-pointer min-h-20 bg-slate-300"
  onclick={startAddGame}
>
  New Game
</button>
{#if menuOpen}
  <div class="fixed top-0 left-0 w-full h-full p-8 backdrop-blur-md text-white">
    <div class="p-4 h-full bg-slate-400 flex flex-col gap-4">
      <input
        placeholder="Name"
        class="w-full p-4 bg-slate-500"
        bind:value={newName}
      />

      <textarea
        placeholder="Description"
        class="w-full p-4 bg-slate-500 font-mono resize-none"
        bind:value={newDesc}
      ></textarea>
      <div class="flex flex-row gap-4">
        <input
          placeholder="Launch Command"
          class="w-full p-4 bg-slate-500 font-mono"
          bind:value={newLaunch}
        />
        <select onchange={onPresetSelected} class="text-black">
          <option value="steam">Steam</option>
          <option value="exe">(Windows) .exe</option>
          <option value="flatpak">(Linux) Flatpak Steam</option>
        </select>
      </div>
      <div class="flex flex-row gap-4">
        <input
          type="submit"
          class="p-4 w-full bg-green-400 cursor-pointer"
          value={editing ? "Confirm Changes" : "Add New Game"}
          onclick={editing ? editGame : addGame}
        />
        <input
          type="submit"
          class="p-4 w-full bg-red-400 cursor-pointer"
          value="Cancel"
          onclick={() => (menuOpen = false)}
        />
      </div>
    </div>
  </div>
{/if}
